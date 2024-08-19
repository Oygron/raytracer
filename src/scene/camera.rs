use std::f64::consts::PI;

use crate::coord::Vec3d;

pub struct Camera{
    pos: Vec3d,
    dir: Vec3d,
    px_down: Vec3d,
    px_left: Vec3d,
    resolution: (u32, u32),
}

pub struct Ray{
    pub start: Vec3d,
    pub dir: Vec3d,
}

impl Camera {
    pub fn new(pos: Vec3d, dir: Vec3d, up: Option<Vec3d>, resolution: Option<(u32, u32)>, fov: Option<f64>) -> Camera {
        let dir = dir.normalize().unwrap();
        let up = up.unwrap_or_else(|| Vec3d{x:0., y:0., z:1.}).normalize().unwrap();
        let resolution = resolution.unwrap_or_else(|| (1024, 768));
        let fov = fov.unwrap_or_else(|| 90.);
        
        //Vecteur orthogonal = up - (projection de up sur dir), 
        let up = (&up - &(up.dot(&dir) * &dir)).normalize().unwrap();
        let right = dir.cross(&up);

        let delta = (fov * PI / 180.)/resolution.0 as f64;
        let px_down = &up * -delta;
        let px_left = &right * -delta;
        
        Camera{
            pos, 
            dir, 
            px_down,
            px_left,
            resolution, 
        }
    }

    pub fn ray(&self, px: (u32, u32)) -> Ray{
        let dx = (px.0 as f64) - (self.resolution.0 as f64)/2.0;
        let dy = (px.1 as f64) - (self.resolution.1 as f64)/2.0;

        let px_window = &(&self.dir + &(dy * &self.px_down)) + &(dx * &self.px_left);

        Ray{start: self.pos.clone(), dir: px_window.normalize().unwrap()}
    }

    pub fn width(&self) -> u32 {
        self.resolution.0
    }
    pub fn height(&self) -> u32 {
        self.resolution.1
    }
}