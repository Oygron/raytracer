use approx::{abs_diff_eq, abs_diff_ne};

use super::camera::Ray;
use super::light::LightType;
use super::object::material::{Color, Material};
use super::object::{Intersect, Object};
use super::{Scene, MAX_BOUNCES};

pub fn render(scene: &Scene) -> Vec<f64> {
    let mut data: Vec<f64> =
        Vec::with_capacity(3 * scene.camera.height() as usize * scene.camera.width() as usize);
    for l in 0..scene.camera.height() {
        for c in 0..scene.camera.width() {
            let color = render_pixel(scene, l, c);
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    normalize(data)
}

fn compute_diffuse(scene: &Scene, ray: Ray, i: Intersect) -> Color{
    
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
                    + i.material.diffuse * light.color * light.intensity * factor;
            }
        }
    }
    color
}


fn compute_specular(scene: &Scene, ray: Ray, i: Intersect, depth: u16) -> Color{
    Color{r: 0., g: 0., b: 0.}
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
            color = compute_diffuse(scene, ray, i);
        }, 
        _ => (),
    }

    color

}

fn render_pixel(scene: &Scene, l: u32, c: u32) -> Color {
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
