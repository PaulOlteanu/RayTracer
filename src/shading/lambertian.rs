use crate::collision::Collision;
use crate::ray::Ray;
use crate::shading::Shader;
use crate::util;

use cgmath::Vector3;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vector3<f64>,
}

// TODO: Implement this
impl Shader for Lambertian {
    // Return value is (attenuation, scatter)
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<(Vector3<f64>, Ray)> {
        let target = collision.point + collision.normal + util::point_in_sphere(1.0);
        let scattered = Ray::new(collision.point, target - collision.point);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    }
}
