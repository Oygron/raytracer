use std::f64::consts::PI;
extern crate approx;

use crate::coord::Vec3d;

pub struct Camera {
    pos: Vec3d,
    dir: Vec3d,
    px_down: Vec3d,
    px_left: Vec3d,
    resolution: (u32, u32),
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub start: Vec3d,
    pub dir: Vec3d,
}

impl Camera {
    pub fn new(
        pos: Vec3d,
        dir: Vec3d,
        up: Option<Vec3d>,
        resolution: Option<(u32, u32)>,
        fov: Option<f64>,
    ) -> Camera {
        let dir = dir.normalize().unwrap();
        let up = up
            .unwrap_or(Vec3d {
                x: 0.,
                y: 0.,
                z: 1.,
            })
            .normalize()
            .unwrap();
        let resolution = resolution.unwrap_or((1024, 768));
        let fov = fov.unwrap_or(90.);

        //Vecteur orthogonal = up - (projection de up sur dir),
        let up = (up - (up.dot(dir) * dir)).normalize().unwrap();
        let right = dir.cross(up);

        let delta = (fov * PI / 180.) / resolution.0 as f64;
        let px_down = up * -delta;
        let px_left = right * -delta;

        Camera {
            pos,
            dir,
            px_down,
            px_left,
            resolution,
        }
    }

    pub fn ray(&self, px: (f64, f64)) -> Ray {
        let dx = px.0 - (self.resolution.0 as f64) / 2.0;
        let dy = px.1 - (self.resolution.1 as f64) / 2.0;

        let px_window = self.dir + (dy * self.px_down) + (dx * self.px_left);

        Ray {
            start: self.pos,
            dir: px_window.normalize().unwrap(),
        }
    }

    pub fn width(&self) -> u32 {
        self.resolution.0
    }
    pub fn height(&self) -> u32 {
        self.resolution.1
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn create_default_camera() {
        let pos = Vec3d {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let dir = Vec3d {
            x: 5.,
            y: 0.3,
            z: -1.,
        };
        let cam = Camera::new(pos, dir, None, None, None);
        assert_eq!(cam.pos, pos);
        assert_abs_diff_eq!(cam.dir, dir.normalize().unwrap());
        assert_eq!(cam.width(), 1024);
        assert_eq!(cam.height(), 768);

        assert_abs_diff_eq!(cam.dir.dot(cam.px_down), 0.);
        assert_abs_diff_eq!(cam.dir.dot(cam.px_left), 0.);
        assert_abs_diff_eq!(cam.px_left.dot(cam.px_down), 0.);
        assert_abs_diff_eq!(
            cam.px_down,
            Vec3d {
                x: -0.0002997799304577759,
                y: -1.7986795827466556e-5,
                z: -0.0015042956910371193
            }
        );
        assert_abs_diff_eq!(
            cam.px_left,
            Vec3d {
                x: -9.187362331913159e-5,
                y: 0.00153122705531886,
                z: -2.6610324844426207e-21
            }
        );
    }

    #[test]
    fn create_ray() {
        let pos = Vec3d {
            x: 1.,
            y: 2.,
            z: 3.,
        };
        let dir = Vec3d {
            x: 5.,
            y: 0.3,
            z: -1.,
        };
        let cam = Camera::new(pos, dir, None, None, None);
        let ray = cam.ray((40., 60.));
        assert_abs_diff_eq!(
            ray.dir,
            Vec3d {
                x: 0.8410810520952884,
                y: -0.49454226181177513,
                z: 0.2191132471768342
            }
        );
        assert_eq!(ray.start, pos);
    }
}
