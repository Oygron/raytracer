use face::Face;
use crate::scene::camera::Ray;
use super::{material::Material, Intersect};

pub mod face;

pub struct Rasterized{
    pub faces: Vec<Face>,
    pub material: Material,
}

impl Rasterized {
    pub fn intersect(&self, ray: &Ray) -> Option<Intersect> {
        let intersects: Vec<Intersect>= self.faces.iter().filter_map(|f| f.intersect(ray)).collect();
        let final_intersect = intersects.into_iter().fold(None, |a, b| {
            match a {
                None => Some(b),
                Some(i) => {
                    if i.dist <= b.dist {
                        Some(i)
                    } else {
                        Some(b)
                    }
                }
            }
        });
        match final_intersect { 
            None => None,
            Some(i) => Some(Intersect{pos: i.pos, dist: i.dist, normal: i.normal, material: self.material.clone()})
        }
    }
}