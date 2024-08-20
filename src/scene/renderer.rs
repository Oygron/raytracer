use super::Scene;
use super::object::Object;
use super::object::material::Color;
use super::light::LightType;



pub fn render(scene: &Scene) -> Vec<f64> {
    let mut data: Vec<f64> = vec![];
    data.reserve(3 * scene.camera.height() as usize * scene.camera.width() as usize);
    for l in 0..scene.camera.height() {
        for c in 0..scene.camera.width(){
            let color = render_pixel(scene, l, c);
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }
    }

    normalize(data)
}

fn render_pixel(scene: &Scene, l: u32, c: u32) -> Color{
    let ray = scene.camera.ray((c, l));

    let mut intersect_dist: f64 = f64::INFINITY;
    let mut color: Option<Color> = None;

    for object in scene.objects.iter() {
        match object{
            Object::Sphere(s) => {
                match s.intersect(&ray){
                    Some(i) if i.dist < intersect_dist => {
                        intersect_dist = i.dist;
                        let mut color_tmp = s.material.base * scene.ambiant_light.color * scene.ambiant_light.intensity;
                        for light in scene.lights.iter() {
                            if let LightType::PointLight { pos:light_pos } = light.light_type  {
                                let factor = i.normal.dot(light_pos - i.pos);
                                color_tmp = color_tmp + s.material.base * light.color * light.intensity * factor;
                            }
                        }
                        color = Some(color_tmp);
                        
                    },
                    _ => (),
                }
            },
            _ => todo!(),
        }
    }

    match color{
        Some(c) => c,
        None => scene.ambiant_light.color * scene.ambiant_light.intensity,
    }
}

fn normalize(data: Vec<f64>) -> Vec<f64>{
    let max_intensity = data.iter().cloned().fold(f64::NAN, f64::max);
    data.iter().map(|v| (*v / max_intensity) ).collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_normalize() {//Should not crash
        let vec = vec![0., 2., 1.];
        assert_eq!(normalize(vec), vec![0., 1., 0.5]);
    }
}