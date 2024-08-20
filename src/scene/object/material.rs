use std::ops;

extern crate approx;
use approx::AbsDiffEq;
use std::f64;

#[derive (Debug, PartialEq, Clone, Copy)]
pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive (Debug, PartialEq, Clone)]
pub struct Material{
    pub base: Color,
    pub specular: Color,
    pub reflectivity: f64,
}

impl Material {
    pub fn default() -> Material{
        let base = Color{r:0., g:0., b:0.};
        let specular = Color{r:0., g:0., b:0.};
        let reflectivity = 0.;
        Material{base, specular, reflectivity}
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color{r:self.r + _rhs.r, g: self.g + _rhs.g, b: self.b + _rhs.b}
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color{r:self.r * _rhs.r, g: self.g * _rhs.g, b: self.b * _rhs.b}
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, _rhs: f64) -> Color {
        Color{r:self.r * _rhs, g: self.g * _rhs, b: self.b * _rhs}
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color{r:self * _rhs.r, g: self * _rhs.g, b: self * _rhs.b}
    }
}

impl AbsDiffEq for Color{
    type Epsilon = f64;

    fn default_epsilon() -> f64 {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: f64) -> bool {
        f64::abs_diff_eq(&self.r, &other.r, epsilon) &&
        f64::abs_diff_eq(&self.g, &other.g, epsilon) &&
        f64::abs_diff_eq(&self.b, &other.b, epsilon)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx::{assert_abs_diff_eq};


    #[test]
    fn default_material() {//Should not crash
        let _ = Material::default();
    }

    #[test]
    fn add_color() {
        let light1 = Color{r:0.1, g:0.2, b:0.3};
        let light2 = Color{r:0.4, g:0.6, b:0.8};
        assert_eq!(light1 + light2, Color{r: 0.5, g:0.8, b:1.1});
    }

    #[test]
    fn mul_colors() {
        let surface = Color{r:0.1, g:0.2, b:0.3};
        let light = Color{r:0.4, g:0.6, b:0.8};
        assert_abs_diff_eq!(surface * light, Color{r: 0.04, g:0.12, b:0.24});
        
    }

    #[test]
    fn light_intensity() {
        let light = Color{r:0.4, g:0.6, b:0.8};
        let intensity = 2.0;
        assert_abs_diff_eq!(intensity * light, Color{r: 0.8, g:1.2, b:1.6});
        assert_abs_diff_eq!(intensity * light, light * intensity);
    }
}