use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::events::attributes::{Attribute, AttrError};
use quick_xml::name::QName;
use core::f64;
use std::env;
use std::fs;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::str::FromStr;

mod camera;
mod light;
mod object;

use camera::Camera;
use crate::coord::Vec3d;
use object::material::{Color, Material};
use object::Object;
use light::{Light, LightType};
use object::sphere::Sphere;

pub struct Scene{
    camera: Camera,
    ambiant_light: Light,
    lights: Vec<Light>,
    objects: Vec<Object>,
}

impl Scene {
    pub fn load(filename: String) -> Scene {
        let file_content = fs::read_to_string(&filename)
            .expect(&format!("file {} cannot be read (path {})",
                &filename, 
                env::current_dir().unwrap().display()));
        Self::load_from_xml_string(file_content)
    }

    fn load_from_xml_string(file_content: String) -> Scene {
        let mut reader = Reader::from_str(&file_content);
        reader.config_mut().trim_text(true);

        let mut camera: Option<Camera> = None;
        let mut lights: Vec<Light> = vec![];
        let mut objects: Vec<Object> = vec![];
        let mut ambiant_light: Option<Light> = None;

        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                // exits the loop when reaching end of file
                Ok(Event::Eof) => break,
        
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"scene" => (),//Nothing to do, root.
                        b"camera" => camera = Some(Self::read_camera(&mut reader)),
                        b"point_light" => lights.push(Self::read_point_light(&mut reader)),
                        b"ambiant_light" => ambiant_light = Some(Self::read_ambiant_light(&mut reader)),
                        b"sphere" => objects.push(Self::read_sphere(&mut reader)),
                        b"object" => (),//todo!(),
                        _ => (),
                    }
                }
                Ok(Event::End(e)) => println!("Exiting {:?}", String::from_utf8_lossy(e.name().as_ref())),
                _ => (),
            }
            // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
            buf.clear();
        }

        Scene{camera: camera.unwrap(), ambiant_light: ambiant_light.unwrap(), lights, objects}
    }

    fn read_value_as_f64(a: Result<Attribute, AttrError>) -> f64 {
        String::from_utf8_lossy(a.unwrap().value.as_ref()).parse::<f64>().unwrap()
    }

    fn read_vec3d(e: BytesStart) -> Vec3d{
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for a in e.attributes(){
            let a_cloned = a.clone();
            match a.unwrap().key {
                QName(b"x") => x = Self::read_value_as_f64(a_cloned),
                QName(b"y") => y = Self::read_value_as_f64(a_cloned),
                QName(b"z") => z = Self::read_value_as_f64(a_cloned),
                _ => (),
            }

        }
        Vec3d{x, y, z}
    }

    fn read_color(e: BytesStart) -> Color{
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;
        for a in e.attributes(){
            let a_cloned = a.clone();
            match a.unwrap().key {
                QName(b"r") => r = Self::read_value_as_f64(a_cloned),
                QName(b"g") => g = Self::read_value_as_f64(a_cloned),
                QName(b"b") => b = Self::read_value_as_f64(a_cloned),
                _ => (),
            }

        }
        r = r.clamp(0., 1.);
        g = g.clamp(0., 1.);
        b = b.clamp(0., 1.);
        Color{r, g, b}
    }

    fn read_property<T: FromStr>(e: &BytesStart, property_name: &[u8]) -> Option<T> {
        match e.attributes().find(|a| a.as_ref().unwrap().key == QName(property_name)){
            None => None,
            Some(a) => {
                match String::from_utf8_lossy(a.unwrap().value.as_ref()).parse::<T>(){
                    Err(_)=> None,
                    Ok(v) => Some(v),
                } 
            },
        }
    }

    fn read_camera(reader: &mut Reader<&[u8]>) -> Camera {
        let mut buf = Vec::new();
        let mut pos: Option<Vec3d> = None;
        let mut dir: Option<Vec3d> = None;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => panic!("Unexpected EOF"),
                Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"pos" => pos = Some(Self::read_vec3d(e)),
                        b"dir" => dir = Some(Self::read_vec3d(e)),
                        _ => (),
                    }
                },
                Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
                Ok(Event::End(e)) => {
                    match e.name().as_ref(){
                        b"camera" => break,
                        name => panic!("unexpected end {:?}", name),
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Camera::new(pos.unwrap(), dir.unwrap(), None, None, None)
    }

    fn read_point_light(reader: &mut Reader<&[u8]>) -> Light {
        let mut buf = Vec::new();
        let mut pos: Option<Vec3d> = None;
        let mut color: Option<Color> = None;
        let mut intensity = 0.;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => panic!("Unexpected EOF"),
                Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"color" => color = Some(Self::read_color(e)),
                        b"intensity" => intensity = Self::read_property::<f64>(&e, b"i").unwrap(),
                        b"pos" => pos = Some(Self::read_vec3d(e)),
                        _ => (),
                    }
                },
                Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
                Ok(Event::End(e)) => {
                    match e.name().as_ref(){
                        b"ambiant_light" => break,
                        name => panic!("unexpected end {:?}", name),
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Light{color: color.unwrap(), intensity, light_type: LightType::PointLight { pos: pos.unwrap() }}
    }


    fn read_ambiant_light(reader: &mut Reader<&[u8]>) -> Light {
        let mut buf = Vec::new();
        let mut color: Option<Color> = None;
        let mut intensity = 0.;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => panic!("Unexpected EOF"),
                Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"color" => color = Some(Self::read_color(e)),
                        b"intensity" => intensity = Self::read_property::<f64>(&e, b"i").unwrap(),
                        _ => (),
                    }
                },
                Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
                Ok(Event::End(e)) => {
                    match e.name().as_ref(){
                        b"ambiant_light" => break,
                        name => panic!("unexpected end {:?}", name),
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Light{color: color.unwrap(), intensity, light_type: LightType::AmbiantLight}
    }

    fn read_material(reader: &mut Reader<&[u8]>) -> Material {
        let mut buf = Vec::new();
        let mut color: Option<Color> = None;
        let mut specular: Option<Color> = None;
        let mut reflectivity = 0.;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => panic!("Unexpected EOF"),
                Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"color" => color = Some(Self::read_color(e)),
                        b"specular" => specular = Some(Self::read_color(e)),
                        b"reflectivity" => reflectivity = Self::read_property::<f64>(&e, b"r").unwrap(),
                        _ => (),
                    }
                },
                Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
                Ok(Event::End(e)) => {
                    match e.name().as_ref(){
                        b"material" => break,
                        name => panic!("unexpected end {:?}", name),
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Material{base: color.unwrap(), specular: specular.unwrap(), reflectivity}
    }

    fn read_sphere(reader: &mut Reader<&[u8]>) -> Object {
        let mut buf = Vec::new();
        let mut pos: Option<Vec3d> = None;
        let mut r = 0.;
        let mut mat: Option<Material> = None;
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => panic!("Unexpected EOF"),
                Ok(Event::Empty(e)) => {
                    match e.name().as_ref() {
                        b"pos" => pos = Some(Self::read_vec3d(e)),
                        b"radius" => r = Self::read_property::<f64>(&e, b"r").unwrap(),
                        _ => (),
                    }
                },
                Ok(Event::Start(e)) => 
                    match e.name().as_ref() {
                        b"material" => mat = Some(Self::read_material(reader)),
                        name => panic!("unexpected block begin named {:?}", name),
                    }
                Ok(Event::End(e)) => {
                    match e.name().as_ref(){
                        b"sphere" => break,
                        name => panic!("unexpected end {:?}", name),
                    }
                }
                _ => (),
            }
            buf.clear();
        }
        Object::Sphere(Sphere::new(pos.unwrap(), r, mat.unwrap()))
    }


    fn to_png(&self, data: Vec<f64>){
        //Normalisation
        let data: Vec<u8> = data.iter().map(|v| (*v * 255.) as u8 ).collect();

        let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.camera.width(), self.camera.height()); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        //encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        //encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        //let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        //    (0.31270, 0.32900),
        //    (0.64000, 0.33000),
        //    (0.30000, 0.60000),
        //    (0.15000, 0.06000)
        //);
        //encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        // An array containing a RGBA sequence.
        writer.write_image_data(&data).unwrap(); // Save
    }
    pub fn render(&self) {

        let mut data: Vec<f64> = vec![];
        data.reserve(3 * self.camera.height() as usize * self.camera.width() as usize);
        for l in 0..self.camera.height() {
            for c in 0..self.camera.width(){
                let ray = self.camera.ray((c, l));

                let mut intersect_dist: f64 = f64::INFINITY;
                let mut color: Option<Color> = None;

                for object in self.objects.iter() {
                    match object{
                        Object::Sphere(s) => {
                            match s.intersect(&ray){
                                Some((dist, _)) if dist < intersect_dist => {
                                    intersect_dist = dist;
                                    color = Some(s.material.base.clone());
                                },
                                _ => (),
                            }
                        },
                        _ => todo!(),
                    }
                }

                if color == None {
                    color = Some(self.ambiant_light.color.clone());
                    for light in self.lights.iter() {
                        match light.light_type {
                            LightType::AmbiantLight => color = Some(light.color.clone()),
                            _ => (),
                        }
                    }
                }

                if color==None{
                    data.push(0.);//r
                    data.push(0.);//g
                    data.push(0.);//b
                } else {
                    let color = color.unwrap();
                    data.push(color.r);
                    data.push(color.g);
                    data.push(color.b);
                }
            }
        }

        self.to_png(data);

    }
    
}


#[cfg(test)]
mod tests {

    use super::*;
    //use quick_xml::events::Event::Start;

    #[test]
    fn parse_vec() {
        let bs = BytesStart::from_content(
            "pos x=\"1.2\" y=\"3.4\" z=\"5.6\"",
            3
        );
        let vec = Scene::read_vec3d(bs);
        assert_eq!(vec, Vec3d{x:1.2, y:3.4, z:5.6});
    }

    #[test]
    fn parse_color() {
        let bs = BytesStart::from_content(
            "color r=\"0.8\" g=\"0\" b=\"0.1\"",
            5
        );
        let col = Scene::read_color(bs);
        assert_eq!(col, Color{r:0.8, g:0., b:0.1});
    }

    #[test]
    fn parse_funny_color() {
        let bs = BytesStart::from_content(
            "color r=\"2.8\" g=\"-0.5\" toto=\"abc\"",
            5
        );
        let col = Scene::read_color(bs);
        assert_eq!(col, Color{r:1., g:0., b:0.});
    }

    #[test]
    fn parse_multi_attributes() {
        let bs = BytesStart::from_content(
            "color r=\"2.8\" g=\"-0.5\" toto=\"abc\"",
            5
        );
        assert_eq!(Scene::read_property::<f64>(&bs, b"r"), Some(2.8));
        assert_eq!(Scene::read_property::<f64>(&bs, b"b"), None);
        assert_eq!(Scene::read_property::<f64>(&bs, b"toto"), None);
    }

}