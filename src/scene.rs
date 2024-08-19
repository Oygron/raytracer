use quick_xml::reader::Reader;
use quick_xml::events::{Event, BytesStart};
use quick_xml::events::attributes::{Attribute, AttrError};
use quick_xml::name::QName;
use std::env;
use std::fs;


mod camera;
mod light;
mod object;

use camera::Camera;
use crate::coord::Vec3d;
use object::material::Color;
//use light::Light;
//use object::Object;

pub struct Scene{
    camera: Camera,
//    lights: Vec<Light>,
//    objects: Vec<Object>,
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
                        b"point_light" => (),//todo!(),
                        b"ambiant_light" => (),//todo!(),
                        b"sphere" => (),//todo!(),
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

        Scene{camera: camera.unwrap()}
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
        Color{r, g, b}
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
        Camera::new(pos.unwrap(), dir.unwrap())
    }




    pub fn render(&self) {

    }
    
}