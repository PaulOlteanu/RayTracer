use cgmath::Vector3;

use crate::collision::Collision;
use crate::ray::Ray;

mod lambertian;
pub use self::lambertian::Lambertian;

pub trait Shader: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, collision: &Collision, attenuation: Vector3<f64>) -> Vector3<f64>;
}
