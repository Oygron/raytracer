pub mod material;
pub mod rasterized;
pub mod sphere;

use crate::coord::Vec3d;
use material::Material;
use rasterized::Rasterized;
use sphere::Sphere;

#[derive(Debug, PartialEq)]
pub struct Intersect {
    pub pos: Vec3d,
    pub dist: f64,
    pub normal: Vec3d,
    pub material: Material,
}

pub enum Object {
    Sphere(Sphere),
    Rasterized(Rasterized),
}
