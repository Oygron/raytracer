
use crate::coord::Vec3d;
use super::{material::Material, Intersect};
use crate::scene::camera::Ray;

pub struct Sphere{
    pub center: Vec3d,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3d, radius: f64, material: Material) -> Sphere {
        Sphere{center, radius, material}
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersect>{
        //Formule demi-droite
        //(x+k×dx)² + (y+k×dy)² + (z+k×dz)², k ∈ ℝ⁺

        //Équation sphère : (x-x0)² + (y-y0)² + (z-z0)² = (RT+alt)²
        //Equation du point d’intersection k tel que (x+k×dx)² + (y+k×dy)² + (z+k×dz)² = (RT+alt)²
        //->k²×(dx²+dy²+dz²)  +  k×2×(x×dx+y×dy+z×dz)  + (x²+y²+z²-RT²)=0
        //     \     a     /       \        b       /    \     c      /
        let a = ray.dir.normsq();
        let b = 2.0*(ray.start - self.center).dot(ray.dir);
        let c = (ray.start - self.center).normsq() - self.radius*self.radius;
        let k_opt = solve_quadratic(a, b, c);

        match k_opt {
            None => None,
            Some(dist) if dist < 0. => None,
            Some(dist) => {
                let pos = ray.start + (dist * ray.dir);
                let normal = (pos - self.center).normalize().unwrap();
                Some(Intersect{pos, 
                               dist,
                               normal,
                               material: self.material.clone()}
                )
                },
        }
 
    }
}


//solves ax²+bx+c=0
//in case of several solutions, returns the lowest positive one
fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<f64> {
    let delta = b*b-4.*a*c;
    if delta < 0.0 {
        return None;
    }

    let s1 = if a > 0. {
        (-b - delta.sqrt())/(2.*a)
    } else {
        (-b + delta.sqrt())/(2.*a)
    };

    if s1 >= 0. {
        Some(s1)
    } else {
        let s2 = if a>0. {
            (-b + delta.sqrt())/(2.*a)
        } else {
            (-b - delta.sqrt())/(2.*a)
        };

        if s2 < 0. {
            None
        } else {
            Some(s2)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    extern crate approx;
    use approx::assert_abs_diff_eq;

    #[test]
    fn quadratic_no_sol() {
        assert_eq!(solve_quadratic(1., 0., 1.), None);
    }

    #[test]
    fn quadratic_both_positive_sol() {
        assert_eq!(solve_quadratic(-4., 5., -1.), Some(0.25));
    }

    #[test]
    fn quadratic_one_negative_sol() {
        assert_eq!(solve_quadratic(4., -5., -12.), Some(2.4663649828320295));
    }

    #[test]
    fn quadratic_both_negative_sol() {
        assert_eq!(solve_quadratic(4., 5., 1.), None);
    }

    #[test]
    fn unit_sphere_in_front() {
        let sphere = Sphere::new(Vec3d{ x: 2., y: 0., z: 0. },
                                         1.0,
                                         Material::default());
        let ray = Ray{start: Vec3d{ x: 0., y: 0., z: 0. },
                           dir  : Vec3d{ x: 1., y: 0., z: 0. }};

        let i = sphere.intersect(&ray).unwrap();
        assert_eq!(i.dist, 1.0);
        assert_eq!(i.pos, Vec3d{x:1., y:0., z:0.});
    }

    #[test]
    fn inside_unit_sphere() {
        let sphere = Sphere::new(Vec3d{ x: 0., y: 0., z: 0. },
                                         1.0,
                                         Material::default());
        let ray = Ray{start: Vec3d{ x: 0., y: 0., z: 0. },
                           dir  : Vec3d{ x: 1., y: 0., z: 0. }};

        let i = sphere.intersect(&ray).unwrap();
        assert_eq!(i.dist, 1.0);
        assert_eq!(i.pos, Vec3d{x:1., y:0., z:0.});
    }

    #[test]
    fn unit_sphere_above() {
        let sphere = Sphere::new(Vec3d{ x: 2., y: 0., z: 2. },
                                         1.0,
                                         Material::default());
        let ray = Ray{start: Vec3d{ x: 0., y: 0., z: 0. },
                           dir  : Vec3d{ x: 1., y: 0., z: 0. }};

        assert_eq!(sphere.intersect(&ray), None);
    }

    #[test]
    fn unit_sphere_behind() {
        let sphere = Sphere::new(Vec3d{ x: -2., y: 0., z: 0. },
                                         1.0,
                                         Material::default());
        let ray = Ray{start: Vec3d{ x: 0., y: 0., z: 0. },
                           dir  : Vec3d{ x: 1., y: 0., z: 0. }};

        assert_eq!(sphere.intersect(&ray), None);
    }


    #[test]
    fn general_sphere() {
        let sphere = Sphere::new(Vec3d{ x: 8., y: 4., z: 2. },
                                         3.0,
                                         Material::default());
        let ray = Ray{start: Vec3d{ x: 2., y: 3., z: 4. },
                           dir  : Vec3d{ x: 1., y: 0.2, z: -0.3 }.normalize().unwrap()};

        let i = sphere.intersect(&ray).unwrap();
        assert_abs_diff_eq!(i.dist, 3.410205739962394);
        assert_abs_diff_eq!(i.pos, Vec3d{ x: 5.208051705064151, 
                                          y: 3.6416103410128304, 
                                          z: 3.037584488480755 });
    }
}