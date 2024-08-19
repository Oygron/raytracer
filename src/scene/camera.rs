use crate::coord::Vec3d;

pub struct Camera{
    pos: Vec3d,
    dir: Vec3d,
    resolution: (u16,u16),
    aperture: f64,//en degrés
}

pub struct Ray{
    pub start: Vec3d,
    pub dir: Vec3d,
}

impl Camera {
    pub fn new(pos: Vec3d, dir: Vec3d) -> Camera {
        Camera{pos, dir, resolution:(1024,768), aperture:90.}
    }
}