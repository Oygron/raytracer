use std::ops;

extern crate approx;
use approx::{RelativeEq, AbsDiffEq};
use std::f64;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3d {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}


impl Vec3d {
    pub fn normsq(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&self) -> Option<Self> {
        match self.norm() {
            0. => None,
            x => Some(self/x)
        }
        
    }
    pub fn dot(&self, other: Vec3d) -> f64{
        self.x*other.x + self.y*other.y + self.z*other.z
    }
    pub fn cross(&self, other: Vec3d) -> Vec3d{
        Vec3d{
            x: self.y*other.z - other.y*self.z,
            y: self.z*other.x - other.z*self.x,
            z: self.x*other.y - other.x*self.y,
        }
    }
}

impl ops::Add<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    fn add(self, _rhs: &Vec3d) -> Vec3d {
        Vec3d{x:self.x + _rhs.x, y: self.y + _rhs.y, z: self.z + _rhs.z}
    }
}

impl ops::Sub<&Vec3d> for &Vec3d {
    type Output = Vec3d;

    fn sub(self, _rhs: &Vec3d) -> Vec3d {
        Vec3d{x:self.x - _rhs.x, y: self.y - _rhs.y, z: self.z - _rhs.z}
    }
}

impl ops::Mul<f64> for &Vec3d {
    type Output = Vec3d;

    fn mul(self, _rhs: f64) -> Vec3d {
        Vec3d{x:self.x * _rhs, y: self.y * _rhs, z: self.z * _rhs}
    }
}

impl ops::Mul<&Vec3d> for f64 {
    type Output = Vec3d;

    fn mul(self, _rhs: &Vec3d) -> Vec3d {
        Vec3d{x:self * _rhs.x, y: self * _rhs.y, z: self * _rhs.z}
    }
}

impl ops::Div<f64> for &Vec3d {
    type Output = Vec3d;

    fn div(self, _rhs: f64) -> Vec3d {
        Vec3d{x:self.x / _rhs, y: self.y / _rhs, z: self.z / _rhs}
    }
}

impl AbsDiffEq for Vec3d{
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.x, &other.x, epsilon) &&
        f64::abs_diff_eq(&self.y, &other.y, epsilon) &&
        f64::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl RelativeEq for Vec3d {
    fn default_max_relative() -> f64 {
        f64::default_max_relative()
    }

    fn relative_eq(&self, other: &Self, epsilon: f64, max_relative: f64) -> bool {
        f64::relative_eq(&self.x, &other.x, epsilon, max_relative) &&
        f64::relative_eq(&self.y, &other.y, epsilon, max_relative) &&
        f64::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn access_vec_coord() {
        let pt = Vec3d{x:1., y:2., z:3.};
        assert_eq!(pt.x, 1.);
        assert_eq!(pt.y, 2.);
        assert_eq!(pt.z, 3.);
    }

    #[test]
    fn add_vec() {
        let pt = Vec3d{x:1., y:2., z:3.};
        let vec = Vec3d{x:-3., y:5., z:7.};
        assert_eq!(&pt + &vec, Vec3d{x: -2., y:7., z:10.});
    }

    #[test]
    fn sub_vec() {
        let pt = Vec3d{x:1., y:2., z:3.};
        let vec = Vec3d{x:-3., y:5., z:7.};
        assert_eq!(&pt - &vec, Vec3d{x: 4., y:-3., z:-4.});
    }

    #[test]
    fn mult_vec() {
        let vec = Vec3d{x:1., y:2., z:3.};
        assert_eq!(2. * &vec, Vec3d{x: 2., y:4., z:6.});
        assert_eq!(2. * &vec, &vec * 2.);
    }

    #[test]
    fn div_vec() {
        let vec = Vec3d{x:1., y:2., z:3.};
        assert_eq!(&vec / 2., Vec3d{x: 0.5, y:1., z:1.5});
    }

    #[test]
    fn normsq_vec(){
        let vec = Vec3d{x:1., y:2., z:3.};
        assert_eq!(vec.normsq(), 14.);
    }

    #[test]
    fn norm_vec(){
        let vec = Vec3d{x:1., y:2., z:3.};
        assert_relative_eq!(vec.norm(), 3.7416573867739413);
    }

    #[test]
    fn normalized_vec(){
        let vec = Vec3d{x:1., y:2., z:3.};
        assert_relative_eq!(vec.normalize().unwrap(), Vec3d{x: 0.2672612419124244, 
                                                             y: 0.5345224838248488, 
                                                             z: 0.8017837257372732});
    }

    #[test]
    fn normalized_null_vec(){
        let vec = Vec3d{x:0., y:0., z:0.};
        assert_eq!(vec.normalize(), None);
    }

    #[test]
    fn dot_vec_gene() {
        let vec1 = Vec3d{x:1., y:2., z:3.};
        let vec2 = Vec3d{x:-3., y:5., z:7.};
        assert_eq!(vec1.dot(vec2), 28.);
    }

    #[test]
    fn dot_ortho() {
        let vec1 = Vec3d{x:1., y:0., z:0.};
        let vec2 = Vec3d{x:0., y:1., z:0.};
        assert_eq!(vec1.dot(vec2), 0.);
    }

    #[test]
    fn cross_ortho() {
        let vec1 = Vec3d{x:1., y:0., z:0.};
        let vec2 = Vec3d{x:0., y:1., z:0.};
        assert_eq!(vec1.cross(vec2), Vec3d{x:0., y:0., z:1.});
    }
}
