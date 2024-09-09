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

#[cfg(test)]
mod tests {

    use crate::coord::Vec3d;

    use super::*;
    fn create_object() -> Rasterized {
        let f1 = Face::new(
            Vec3d{x: 1., y:  1., z: -1.},
            Vec3d{x: 1., y:  0., z:  1.},
            Vec3d{x: 1., y: -1., z: -1.}
        );
        let f2 = Face::new(
            Vec3d{x: 2., y:  1., z: -1.},
            Vec3d{x: 2., y:  0., z:  1.},
            Vec3d{x: 2., y: -1., z: -1.}
        );
        let f3 = Face::new(
            Vec3d{x: 0.5, y:  1., z: -1.},
            Vec3d{x: 0.5, y:  0., z:  1.},
            Vec3d{x: 0.5, y: -1., z: -1.}
        );
        Rasterized{faces: vec![f1, f2, f3], material: Material::default()}
    }

    #[test]
    fn face_normal(){
        let object = create_object();
        let ray = Ray {
            start: Vec3d {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            dir: Vec3d {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        };
        let i = object.intersect(&ray).unwrap();
        assert_eq!(
            i.pos, 
            Vec3d {
                x: 1.,
                y: 0.,
                z: 0.
            }
        )
    }

    #[test]
    fn face_missed(){
        let object = create_object();
        let ray = Ray {
            start: Vec3d {
                x: 0.,
                y: 0.,
                z: 2.,
            },
            dir: Vec3d {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        };
        assert_eq!(object.intersect(&ray), None);
    }
}