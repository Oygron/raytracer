pub mod sphere;
pub mod material;
pub mod rasterized;

use sphere::Sphere;
use material::Material;

pub enum Object{
    Sphere(Sphere),
    Rasterized,
}
