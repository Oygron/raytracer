pub struct Color{
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
pub struct Material{
    base: Color,
    specular: Color,
    reflectivity: f64,
}

impl Material {
    pub fn default() -> Material{
        let base = Color{r:0., g:0., b:0.};
        let specular = Color{r:0., g:0., b:0.};
        let reflectivity = 0.;
        Material{base, specular, reflectivity}
    }
}