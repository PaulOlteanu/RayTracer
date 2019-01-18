use crate::collision::Collision;
use crate::ray::Ray;
use crate::shading::Shader;
use crate::util;

use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
pub struct Metal {
    pub albedo: Vector3<f64>,
}

// TODO: Implement this
impl Shader for Metal {
    // Return value is (attenuation, scatter)
    fn scatter(
        &self,
        ray: &Ray,
        collision: &Collision,
    ) -> Option<(Vector3<f64>, Ray)> {
        let reflected = reflect(ray.direction().normalize(), collision.normal);
        let scattered = Ray::new(collision.point, reflected);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}

fn reflect(in_vec: Vector3<f64>, norm: Vector3<f64>) -> Vector3<f64> {
    in_vec - 2.0 * norm.dot(in_vec) * norm
}
