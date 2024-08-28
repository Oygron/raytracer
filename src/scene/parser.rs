use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::reader::Reader;

use std::str::FromStr;

use super::camera::Camera;
use super::light::{Light, LightType};
use super::object::material::{Color, Material};
use super::object::sphere::Sphere;
use super::object::Object;
use crate::coord::Vec3d;

use super::Scene;

pub fn load_from_xml_string(file_content: String) -> Scene {
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
                    b"scene" => (), //Nothing to do, root.
                    b"camera" => camera = Some(read_camera(&mut reader)),
                    b"point_light" => lights.push(read_point_light(&mut reader)),
                    b"ambiant_light" => ambiant_light = Some(read_ambiant_light(&mut reader)),
                    b"sphere" => objects.push(read_sphere(&mut reader)),
                    b"object" => (), //todo!(),
                    _ => (),
                }
            }
            Ok(Event::End(e)) => {
                println!("Exiting {:?}", String::from_utf8_lossy(e.name().as_ref()))
            }
            _ => (),
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    Scene {
        camera: camera.unwrap(),
        ambiant_light: ambiant_light.unwrap(),
        lights,
        objects,
    }
}

fn read_value_as_f64(a: &[u8]) -> f64 {
    String::from_utf8_lossy(a).parse::<f64>().unwrap()
}

fn read_vec3d(e: BytesStart) -> Vec3d {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;
    for a in e.attributes() {
        let Attribute { key: k, value } = a.unwrap();
        match k {
            QName(b"x") => x = read_value_as_f64(&value),
            QName(b"y") => y = read_value_as_f64(&value),
            QName(b"z") => z = read_value_as_f64(&value),
            _ => (),
        }
    }
    Vec3d { x, y, z }
}

fn read_color(e: BytesStart) -> Color {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    for a in e.attributes() {
        let Attribute { key: k, value } = a.unwrap();
        match k {
            QName(b"r") => r = read_value_as_f64(&value),
            QName(b"g") => g = read_value_as_f64(&value),
            QName(b"b") => b = read_value_as_f64(&value),
            _ => (),
        }
    }
    r = r.clamp(0., 1.);
    g = g.clamp(0., 1.);
    b = b.clamp(0., 1.);
    Color { r, g, b }
}

fn read_property<T: FromStr>(e: &BytesStart, property_name: &[u8]) -> Option<T> {
    match e
        .attributes()
        .find(|a| a.as_ref().unwrap().key == QName(property_name))
    {
        None => None,
        Some(a) => match String::from_utf8_lossy(a.unwrap().value.as_ref()).parse::<T>() {
            Err(_) => None,
            Ok(v) => Some(v),
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
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"pos" => pos = Some(read_vec3d(e)),
                b"dir" => dir = Some(read_vec3d(e)),
                _ => (),
            },
            Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"camera" => break,
                name => panic!("unexpected end {:?}", name),
            },
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
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"color" => color = Some(read_color(e)),
                b"intensity" => intensity = read_property::<f64>(&e, b"i").unwrap(),
                b"pos" => pos = Some(read_vec3d(e)),
                _ => (),
            },
            Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"point_light" => break,
                name => panic!("unexpected end {:?}", name),
            },
            _ => (),
        }
        buf.clear();
    }
    Light {
        color: color.unwrap(),
        intensity,
        light_type: LightType::PointLight { pos: pos.unwrap() },
    }
}

fn read_ambiant_light(reader: &mut Reader<&[u8]>) -> Light {
    let mut buf = Vec::new();
    let mut color: Option<Color> = None;
    let mut intensity = 0.;
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
            Ok(Event::Eof) => panic!("Unexpected EOF"),
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"color" => color = Some(read_color(e)),
                b"intensity" => intensity = read_property::<f64>(&e, b"i").unwrap(),
                _ => (),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"ambiant_light" => (), //tests will have that line
                name => panic!("unexpected block begin named {:?}", name),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"ambiant_light" => break,
                name => panic!("unexpected end {:?}", name),
            },
            _ => (),
        }
        buf.clear();
    }
    Light {
        color: color.unwrap(),
        intensity,
        light_type: LightType::AmbiantLight,
    }
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
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"color" => color = Some(read_color(e)),
                b"specular" => specular = Some(read_color(e)),
                b"reflectivity" => reflectivity = read_property::<f64>(&e, b"r").unwrap(),
                _ => (),
            },
            Ok(Event::Start(e)) => panic!("unexpected block begin named {:?}", e.name().as_ref()),
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"material" => break,
                name => panic!("unexpected end {:?}", name),
            },
            _ => (),
        }
        buf.clear();
    }
    Material {
        base: color.unwrap(),
        specular: specular.unwrap(),
        reflectivity,
    }
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
            Ok(Event::Empty(e)) => match e.name().as_ref() {
                b"pos" => pos = Some(read_vec3d(e)),
                b"radius" => r = read_property::<f64>(&e, b"r").unwrap(),
                _ => (),
            },
            Ok(Event::Start(e)) => match e.name().as_ref() {
                b"material" => mat = Some(read_material(reader)),
                name => panic!("unexpected block begin named {:?}", name),
            },
            Ok(Event::End(e)) => match e.name().as_ref() {
                b"sphere" => break,
                name => panic!("unexpected end {:?}", name),
            },
            _ => (),
        }
        buf.clear();
    }
    Object::Sphere(Sphere::new(pos.unwrap(), r, mat.unwrap()))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_vec() {
        let bs = BytesStart::from_content("pos x=\"1.2\" y=\"3.4\" z=\"5.6\"", 3);
        let vec = read_vec3d(bs);
        assert_eq!(
            vec,
            Vec3d {
                x: 1.2,
                y: 3.4,
                z: 5.6
            }
        );
    }

    #[test]
    fn parse_color() {
        let bs = BytesStart::from_content("color r=\"0.8\" g=\"0\" b=\"0.1\"", 5);
        let col = read_color(bs);
        assert_eq!(
            col,
            Color {
                r: 0.8,
                g: 0.,
                b: 0.1
            }
        );
    }

    #[test]
    fn parse_funny_color() {
        let bs = BytesStart::from_content("color r=\"2.8\" g=\"-0.5\" toto=\"abc\"", 5);
        let col = read_color(bs);
        assert_eq!(
            col,
            Color {
                r: 1.,
                g: 0.,
                b: 0.
            }
        );
    }

    #[test]
    fn parse_multi_attributes() {
        let bs = BytesStart::from_content("color r=\"2.8\" g=\"-0.5\" toto=\"abc\"", 5);
        assert_eq!(read_property::<f64>(&bs, b"r"), Some(2.8));
        assert_eq!(read_property::<f64>(&bs, b"b"), None);
        assert_eq!(read_property::<f64>(&bs, b"toto"), None);
    }

    #[test]
    fn parse_ambiant_light() {
        let mut reader = Reader::from_str(
            "<ambiant_light>
            <color r=\"0.6\" g=\"0.8\" b=\"1\"/>
            <intensity i=\"0.3\"/>
            </ambiant_light>",
        );

        let light = read_ambiant_light(&mut reader);
        assert_eq!(
            light.color,
            Color {
                r: 0.6,
                g: 0.8,
                b: 1.0
            }
        );
        assert_eq!(light.intensity, 0.3);
    }

    //TODO: faire les autres parseurs
}
