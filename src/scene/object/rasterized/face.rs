use coords::{Point, Vector};

pub struct Face {
    coords: [Point; 3],
}

impl Face {
    pub fn new(a: Point, b: Point, c: Point) -> Face {

    }
    pub fn intersect_dist(&self, ray:(Point, Vector)) -> f64 {

    }
    pub fn normal(&self) -> Vector{
        
    }
}