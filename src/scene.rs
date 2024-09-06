use core::f64;
use std::env;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

use camera::Camera;
use light::Light;
use object::Object;

mod camera;
mod light;
mod object;
mod parser;
mod renderer;

pub const MAX_BOUNCES:u16 = 3;

pub struct Scene {
    pub camera: Camera,
    pub ambiant_light: Light,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn load(filename: String) -> Scene {
        let file_content = fs::read_to_string(&filename).expect(&format!(
            "file {} cannot be read (path {})",
            &filename,
            env::current_dir().unwrap().display()
        ));
        parser::load_from_xml_string(file_content)
    }

    fn to_png(&self, data: Vec<f64>) {
        //-> [0..255]
        let data: Vec<u8> = data.iter().map(|v| (*v * 255.) as u8).collect();

        let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.camera.width(), self.camera.height()); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        // An array containing a RGBA sequence.
        writer.write_image_data(&data).unwrap(); // Save
    }

    pub fn render(&self) {
        //let data: Vec<f64> = renderer::render(self);
        //let data: Vec<f64> = renderer::render_multitrhead(self);
        let data: Vec<f64> = renderer::render_rayon(self);
        self.to_png(data);
    }
}
