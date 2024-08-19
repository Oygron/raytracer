use crate::coord::Vec3d;

use super::object::material::Color;


pub enum LightType{
    AmbiantLight,
    PointLight{pos: Vec3d}
}
pub struct Light{
    pub color : Color,
    pub intensity: f64,
    pub light_type: LightType,
}