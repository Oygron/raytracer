
use crate::{coord::Vec3d, scene::{camera::Ray, object::{material::Material, Intersect}}};

#[derive(Clone, Copy)]
pub struct Face {
    coords: [Vec3d; 3],
    normal: Vec3d,
    inside_vecs: [Vec3d; 3],
}

impl Face {
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Face {
        let normal = Face::compute_normal(a, b, c);
        Face{
            coords: [a, b, c], 
            normal,
            inside_vecs :
                [(b - a).cross(normal),
                 (c - b).cross(normal),
                 (a - c).cross(normal)],
        }
    }
    pub fn intersect(&self, ray:&Ray) -> Option<Intersect> {
        //ray is parallel to the face
        if ray.dir.dot(self.normal) == 0. {
            return None;
        }
        //ray would hit face from the back
        if ray.dir.dot(self.normal) > 0. {
            return None;
        }
        
        let [a, b, c] = self.coords;

        //plane equation : Nx(x - Ax) + Ny(y - Ay) + Nz(z - Az) = 0
        //ray equation (x0 + k.dx, y0 + k.dy, z0 + k.dz)
        // trouver k tel que 
        //Nx(x0 + k.dx - Ax) + Ny(y0 + k.dy - Ay) + Nz(y0 + k.dz - Az) = 0
        //k(Nx.dx + Ny.dy + Nz.dz) = Nx(Ax - x0) + Ny(Ay - y0) + Nz(Az - z0)


        //find coordinates of intersection with the whole plane
        let r0 = ray.start;
        let n = self.normal;
        let dist = (n * (a - r0)).sum()/n.dot(ray.dir);
        
        let intersect_point = ray.start + dist * ray.dir;

        //plane is behind the source
        if dist <= 0. {
            return None;
        }
        

        //find if intersection is inside the triangle
        let ab_side = self.inside_vecs[0].dot(intersect_point - a);
        let bc_side = self.inside_vecs[1].dot(intersect_point - b);
        let ca_side = self.inside_vecs[2].dot(intersect_point - c);

        if ab_side < 0. || bc_side < 0. || ca_side < 0. {
            None
        } else {
            Some(Intersect{
                pos: intersect_point, 
                dist, 
                normal: self.normal, 
                material: Material::default()})
        }
    }

    pub fn compute_normal(a: Vec3d, b: Vec3d, c: Vec3d) -> Vec3d{
        (b - a).cross(b - c).normalize().unwrap()
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    fn create_face() -> Face {
        Face::new(
            Vec3d{x: 1., y:  1., z: -1.},
            Vec3d{x: 1., y:  0., z:  1.},
            Vec3d{x: 1., y: -1., z: -1.}
        ) 
    }

    #[test]
    fn face_normal(){
        let face = create_face();
        assert_eq!(
            face.normal,
            Vec3d {
                x: -1.,
                y:  0.,
                z:  0.,
            },
        )
    }

    #[test]
    fn face_in_front() {
        let face = create_face();
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
        let i = face.intersect(&ray).unwrap();
        assert_eq!(i.dist, 1.0);
        assert_eq!(
            i.pos,
            Vec3d {
                x: 1.,
                y: 0.,
                z: 0.
            }
        );
    }

    #[test]
    fn face_behind() {
        let face = create_face();
        let ray = Ray {
            start: Vec3d {
                x: 2.,
                y: 0.,
                z: 0.,
            },
            dir: Vec3d {
                x:  1.,
                y:  0.,
                z:  0.,
            },
        };
        assert_eq!(face.intersect(&ray), None);
    }

    #[test]
    fn parallel_face() {
        let face = create_face();
        let ray = Ray {
            start: Vec3d {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            dir: Vec3d {
                x: 0.,
                y: 0.,
                z: 1.,
            },
        };
        assert_eq!(face.intersect(&ray), None);
    }

    #[test]
    fn outside_face() {
        let face = create_face();
        let ray = Ray {
            start: Vec3d {
                x: 0.,
                y: 0.75,
                z: 0.,
            },
            dir: Vec3d {
                x: 1.,
                y: 0.,
                z: 0.,
            },
        };
        assert_eq!(face.intersect(&ray), None);
    }

    #[test]
    fn back_face() {
        let face = create_face();
        let ray = Ray {
            start: Vec3d {
                x: 2.,
                y: 0.,
                z: 0.,
            },
            dir: Vec3d {
                x:-1.,
                y: 0.,
                z: 0.,
            },
        };
        assert_eq!(face.intersect(&ray), None);
    }
}