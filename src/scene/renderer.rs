use std::f64::consts::PI;
use std::sync::mpsc::channel;
use threadpool::ThreadPool;
use threadpool_scope::scope_with;
use rayon::prelude::*;

use crate::coord::Vec3d;

use super::camera::Ray;
use super::light::LightType;
use super::object::material::Color;
use super::object::{Intersect, Object};
use super::Scene;
use super::MAX_BOUNCES;

#[cfg(debug_assertions)]
const NB_ITER: usize = 2;

#[cfg(not(debug_assertions))]
const NB_ITER: usize = 100;

const NB_WORKERS: usize = 4;

impl Scene{
    pub fn render_rayon(&self) -> Vec<f64> {

        let mut data: Vec<f64> =
            Vec::with_capacity(3 * self.camera.height() as usize * self.camera.width() as usize);
        
        for l in 0..self.camera.height() {
            for c in 0..self.camera.width() {
                let color = 
                    (0..NB_ITER)
                    .into_par_iter()
                    .map(|_| self.render_pixel(l, c))
                    .reduce(|| Color{r:0., g:0., b:0.}, 
                    |a, b| a + b);

                data.push(color.r);
                data.push(color.g);
                data.push(color.b);
            }
        }

        Self::normalize(data)
    }

    pub fn render_multithread(&self) -> Vec<f64> {
        let pool = ThreadPool::new(NB_WORKERS);
    
        let mut data: Vec<f64> =
            Vec::with_capacity(3 * self.camera.height() as usize * self.camera.width() as usize);
        for l in 0..self.camera.height() {
            for c in 0..self.camera.width() {
                let color = scope_with( &pool, |scope| {
                    let (tx, rx) = channel();
                    for _ in 0..NB_ITER {
                        let tx = tx.clone();
                        scope.execute(move|| {
                            tx.send(self.render_pixel(l, c)).expect("channel will be there waiting for the pool");
                        });
                    }
                    rx.iter().take(NB_ITER).fold(Color{r:0., g:0., b:0.}, |a, b| a+b)
                });
    
                data.push(color.r);
                data.push(color.g);
                data.push(color.b);
            }
        }
    
        Self::normalize(data)
    }
    
    pub fn render_monothread(&self) -> Vec<f64> {
        let mut data: Vec<f64> =
            Vec::with_capacity(3 * self.camera.height() as usize * self.camera.width() as usize);
        for l in 0..self.camera.height() {
            for c in 0..self.camera.width() {
                let mut color = Color{r:0., g:0., b:0.};
                for _ in 0..NB_ITER {
                    color = color + self.render_pixel(l, c);
                }
    
                data.push(color.r);
                data.push(color.g);
                data.push(color.b);
            }
        }
    
        Self::normalize(data)
    }

    fn render_pixel(&self, l: u32, c: u32) -> Color {

        let c = (c as f64) + rand::random::<f64>() - 0.5;
        let l = (l as f64) + rand::random::<f64>() - 0.5;
    
        let ray = self.camera.ray((c, l));
    
        self.send_ray(ray, MAX_BOUNCES)
    }

    fn compute_diffuse(&self, i: &Intersect) -> Color{
    
        let mut color =
            i.material.diffuse * self.ambiant_light.color * self.ambiant_light.intensity;
    
        for light in self.lights.iter() {
            if let LightType::PointLight { pos: light_pos } = light.light_type {
                let light_dir = (i.pos - light_pos).normalize().unwrap();
                let light_ray = Ray{start: light_pos, dir: light_dir};
                let light_intersect = self.get_intersect(light_ray);
                let dist_light = (light_pos - i.pos).norm();
    
                //Intersect sooner
                if let Some(Intersect{pos:_, dist: d, normal:_, material:_}) = light_intersect {
                    if d < dist_light - 1e-9 {
                        continue;
                    }
                } else {
                    //No intersect (should not happen except rounding error)
                    continue;
                }
                
                let factor = i.normal.dot(light_pos - i.pos);
                if factor > 0. {
                    color = color
                        + i.material.diffuse * light.color * light.intensity * (factor/(dist_light*dist_light));
                }
            }
        }
        color
    }
    
    
    fn compute_reflection(&self, ray: Ray, i: &Intersect, depth: u16) -> Color{
        
        
        let r = rand::random::<f64>() * i.material.roughness * PI / 2.;
        let alpha = rand::random::<f64>() * 2. * PI;
    
        let dir = ray.dir.symmetry(i.normal) * -1.;
    
        //first orthogonal
        let norm1: Vec3d;
        //x le plus petit
        if dir.x.abs() < dir.y.abs() &&
           dir.x.abs() < dir.z.abs(){
            norm1 = Vec3d{x:0., y:-dir.z, z:dir.y}.normalize().unwrap();
        } else if dir.y.abs() < dir.z.abs(){//y le plus petit
            norm1 = Vec3d{x:-dir.z, y:0., z:dir.x}.normalize().unwrap();
        } else {//z le plus petit
            norm1 = Vec3d{x:-dir.y, y:dir.x, z:0.}.normalize().unwrap();
        }
    
        let norm2 = dir.cross(norm1);
    
        let norm = alpha.cos()*norm1 + alpha.sin()*norm2;
    
        let dir = r.cos()*dir + r.sin()*norm;
    
        let symmetric_ray = Ray{start:i.pos, dir};
    
        self.send_ray(symmetric_ray, depth-1)*i.material.specular
    }
    
    fn compute_specular(&self, ray: Ray, i: &Intersect) -> Color{
        let symmetric_ray = Ray{start:i.pos, dir:ray.dir.symmetry(i.normal) * -1.};
        //let mut color = Color{r:0., g:0., b:0.};
        let mut color = i.material.specular * self.ambiant_light.color * self.ambiant_light.intensity;
    
    
        for light in self.lights.iter() {
            if let LightType::PointLight { pos: light_pos } = light.light_type {
                let light_dir = (i.pos - light_pos).normalize().unwrap();
                let light_ray = Ray{start: light_pos, dir: light_dir};
                let light_intersect = self.get_intersect(light_ray);
                let dist_light = (light_pos - i.pos).norm();
    
                //Intersect sooner
                if let Some(Intersect{pos:_, dist: d, normal:_, material:_}) = light_intersect {
                    if d < dist_light - 1e-9 {
                        continue;
                    }
                } else {
                    //No intersect (should not happen except rounding error)
                    continue;
                }
    
                let factor = symmetric_ray.dir.dot(light_dir * -1.);
                if factor > 0. {
                    let angle = factor.acos();
                    let angle = angle/i.material.roughness;
                    if angle > PI/2. {
                        continue;
                    }
                    let factor = angle.cos(); 
                    color = color
                        + i.material.specular * light.color * light.intensity * (factor/(dist_light*dist_light*i.material.roughness));
                }
            }
        }
        color
    }
    
    fn get_intersect(&self, ray: Ray) -> Option<Intersect>{
        let mut ray_intersect : Option<Intersect> = None;
        let mut intersect_dist: f64 = f64::INFINITY;
    
        for object in self.objects.iter() {
            match object {
                Object::Sphere(s) => match s.intersect(&ray) {
                    Some(i) if i.dist < intersect_dist => {
                        intersect_dist = i.dist;
                        ray_intersect = Some(i);
                    }
                    _ => (),
                },
                Object::Rasterized(r) => match r.intersect(&ray){
                    Some(i) if i.dist < intersect_dist => {
                        intersect_dist = i.dist;
                        ray_intersect = Some(i);
                    }
                    _ => (),
                },
            }
        }
    
        ray_intersect
    }
    
    fn send_ray(&self, ray: Ray, depth: u16) -> Color {
        let mut color: Color = self.ambiant_light.color * self.ambiant_light.intensity;
        if depth == 0 {
            return color;
        }
    
        let ray_intersect = self.get_intersect(ray);
    
        match ray_intersect {
            Some(i)=>{
                color = self.compute_diffuse(&i) * (1. - i.material.reflectivity);
                if i.material.reflectivity > 0. {
                    color = color + self.compute_reflection(ray, &i, depth) * i.material.reflectivity;
                    color = color + self.compute_specular(ray, &i) * i.material.reflectivity;
                }
            }, 
            _ => (),
        }
    
        color
    
    }
    
    
    fn normalize(data: Vec<f64>) -> Vec<f64> {
        let max_intensity = data.iter().cloned().fold(f64::NAN, f64::max);
        data.iter().map(|v| (*v / max_intensity)).collect()
    }

}



#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_normalize() {
        let vec = vec![0., 2., 1.];
        assert_eq!(Scene::normalize(vec), vec![0., 1., 0.5]);
    }

    use crate::scene::{camera::Camera, light::Light, object::{material::Material, sphere::Sphere}};

    fn create_empty_scene() -> Scene{
        let camera = Camera::new(
            Vec3d{x:0., y:0., z:0.},
            Vec3d{x:1., y:0., z:0.},
            None, 
            Some((1,1)), 
            None
        );
        let ambiant_light = Light{
            color: Color{r:1., g:0., b:0.},
            intensity: 1.,
            light_type: LightType::AmbiantLight,
        };
        Scene { 
            camera: camera, 
            ambiant_light: ambiant_light, 
            lights: vec![], 
            objects: vec![],
        }
    }

    #[test]
    fn test_monothread(){
        let scene = create_empty_scene();
        let colors = scene.render_monothread();
        assert_eq!(colors[0], 1.);
        assert_eq!(colors[1], 0.);
        assert_eq!(colors[2], 0.);
    }

    #[test]
    fn test_multithread(){
        let scene = create_empty_scene();
        let colors = scene.render_multithread();
        assert_eq!(colors[0], 1.);
        assert_eq!(colors[1], 0.);
        assert_eq!(colors[2], 0.);
    }

    #[test]
    fn test_rayon(){
        let scene = create_empty_scene();
        let colors = scene.render_rayon();
        assert_eq!(colors[0], 1.);
        assert_eq!(colors[1], 0.);
        assert_eq!(colors[2], 0.);
    }

    #[test]
    fn test_diffuse(){
        let mut scene = create_empty_scene();
        scene.lights.push(
            Light{
                color: Color { r: 0., g: 1., b: 1. },
                intensity: 1.,
                light_type: LightType::PointLight { pos: Vec3d { x: -1., y: 0., z: 0. } }
            }
        );
        let mut sphere_material = Material::default();
        sphere_material.diffuse.g = 1.;
        sphere_material.reflectivity = 0.;
        scene.objects.push(
            Object::Sphere(Sphere{
                center: Vec3d { x: 1., y: 0., z: 0. },
                material: sphere_material.clone(),
                radius: 1.,
            })
        );
        let intersect = Intersect{
            dist: 1.,
            pos: Vec3d { x: 0., y: 0., z: 0. },
            normal: Vec3d { x: 1., y: 0., z: 0. },
            material: sphere_material.clone(),
        };
        let color = scene.compute_diffuse(&intersect);
        assert_eq!(color.r, 0.);
        assert_eq!(color.g, 0.);
        assert_eq!(color.b, 0.);
    }
}
