use cgmath::Vector3;

use crate::collision::Collision;
use crate::ray::Ray;

mod lambertian;
mod metal;
pub use self::lambertian::Lambertian;
pub use self::metal::Metal;

pub trait Shader: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Vector3<f64>, Ray)>;
}
