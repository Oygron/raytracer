use crate::coord::{Point, Vector};

pub struct Camera{
    pos: Point,
    dir: Vector,
}

impl Camera {
    pub fn new(pos: Point, dir: Vector) -> Camera {
        Camera{pos, dir}
    }
}