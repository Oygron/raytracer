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
use super::{Scene, MAX_BOUNCES};

#[cfg(debug_assertions)]
const NB_ITER: usize = 2;

#[cfg(not(debug_assertions))]
const NB_ITER: usize = 100;

const NB_WORKERS: usize = 4;

pub fn render_rayon(scene: &Scene) -> Vec<f64> {

    let mut data: Vec<f64> =
        Vec::with_capacity(3 * scene.camera.height() as usize * scene.camera.width() as usize);
    
    for l in 0..scene.camera.height() {
        for c in 0..scene.camera.width() {
            let color = 
                (0..NB_ITER)
                .into_par_iter()
                .map(|_| render_pixel(scene, l, c))
                .reduce(|| Color{r:0., g:0., b:0.}, 
                |a, b| a + b);

            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    normalize(data)
}

pub fn render_multitrhead(scene: &Scene) -> Vec<f64> {
    let pool = ThreadPool::new(NB_WORKERS);

    let mut data: Vec<f64> =
        Vec::with_capacity(3 * scene.camera.height() as usize * scene.camera.width() as usize);
    for l in 0..scene.camera.height() {
        for c in 0..scene.camera.width() {
            let color = scope_with( &pool, |scope| {
                let (tx, rx) = channel();
                for _ in 0..NB_ITER {
                    let tx = tx.clone();
                    scope.execute(move|| {
                        tx.send(render_pixel(scene, l, c)).expect("channel will be there waiting for the pool");
                    });
                }
                rx.iter().take(NB_ITER).fold(Color{r:0., g:0., b:0.}, |a, b| a+b)
            });

            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    normalize(data)
}

pub fn render(scene: &Scene) -> Vec<f64> {
    let mut data: Vec<f64> =
        Vec::with_capacity(3 * scene.camera.height() as usize * scene.camera.width() as usize);
    for l in 0..scene.camera.height() {
        for c in 0..scene.camera.width() {
            let mut color = Color{r:0., g:0., b:0.};
            for _ in 0..NB_ITER {
                color = color + render_pixel(scene, l, c);
            }

            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    normalize(data)
}

fn compute_diffuse(scene: &Scene, i: &Intersect) -> Color{
    
    let mut color =
        i.material.diffuse * scene.ambiant_light.color * scene.ambiant_light.intensity;

    for light in scene.lights.iter() {
        if let LightType::PointLight { pos: light_pos } = light.light_type {
            let light_dir = (i.pos - light_pos).normalize().unwrap();
            let light_ray = Ray{start: light_pos, dir: light_dir};
            let light_intersect = get_intersect(scene, light_ray);
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


fn compute_reflection(scene: &Scene, ray: Ray, i: &Intersect, depth: u16) -> Color{
    
    
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

    send_ray(scene, symmetric_ray, depth-1)*i.material.specular
}

fn compute_specular(scene: &Scene, ray: Ray, i: &Intersect) -> Color{
    let symmetric_ray = Ray{start:i.pos, dir:ray.dir.symmetry(i.normal) * -1.};
    //let mut color = Color{r:0., g:0., b:0.};
    let mut color = i.material.specular * scene.ambiant_light.color * scene.ambiant_light.intensity;


    for light in scene.lights.iter() {
        if let LightType::PointLight { pos: light_pos } = light.light_type {
            let light_dir = (i.pos - light_pos).normalize().unwrap();
            let light_ray = Ray{start: light_pos, dir: light_dir};
            let light_intersect = get_intersect(scene, light_ray);
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

fn get_intersect(scene: &Scene, ray: Ray) -> Option<Intersect>{
    let mut ray_intersect : Option<Intersect> = None;
    let mut intersect_dist: f64 = f64::INFINITY;

    for object in scene.objects.iter() {
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

fn send_ray(scene: &Scene, ray: Ray, depth: u16) -> Color {
    let mut color: Color = scene.ambiant_light.color * scene.ambiant_light.intensity;
    if depth == 0 {
        return color;
    }

    let ray_intersect = get_intersect(scene, ray);

    match ray_intersect {
        Some(i)=>{
            color = compute_diffuse(scene, &i) * (1. - i.material.reflectivity);
            if i.material.reflectivity > 0. {
                color = color+compute_reflection(scene, ray, &i, depth) * i.material.reflectivity;
                color = color+compute_specular(scene, ray, &i) * i.material.reflectivity;
            }
        }, 
        _ => (),
    }

    color

}

fn render_pixel(scene: &Scene, l: u32, c: u32) -> Color {

    let c = (c as f64) + rand::random::<f64>() - 0.5;
    let l = (l as f64) + rand::random::<f64>() - 0.5;

    let ray = scene.camera.ray((c, l));

    send_ray(scene, ray, MAX_BOUNCES)
}

fn normalize(data: Vec<f64>) -> Vec<f64> {
    let max_intensity = data.iter().cloned().fold(f64::NAN, f64::max);
    data.iter().map(|v| (*v / max_intensity)).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_normalize() {
        //Should not crash
        let vec = vec![0., 2., 1.];
        assert_eq!(normalize(vec), vec![0., 1., 0.5]);
    }
}
