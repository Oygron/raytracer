pub mod sphere;
pub mod material;
pub mod rasterized;

use material::Material;
use sphere::Sphere;
use crate::coord::Vec3d;

#[derive (Debug, PartialEq)]
pub struct Intersect{
    pub pos: Vec3d,
    pub dist: f64,
    pub normal: Vec3d,
    pub material: Material,
}

pub enum Object{
    Sphere(Sphere),
    Rasterized,
}
