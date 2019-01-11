use crate::shading::Shader;
use crate::collision::Collision;
use crate::ray::Ray;

use cgmath::Vector3;

#[derive(Debug)]
pub struct Lambertian {

}

// TODO: Implement this
impl Shader for Lambertian {
    fn scatter(&self, ray: &Ray, collision: &Collision, attenuation: Vector3<f64>) -> Vector3<f64> {
        Vector3::new(0.0, 0.0, 0.0)
    }
}
