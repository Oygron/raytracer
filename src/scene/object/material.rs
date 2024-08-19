#[derive (Debug, PartialEq, Clone)]
pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

#[derive (Debug, PartialEq)]
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_material() {//Should not crash
        let _ = Material::default();
    }
}