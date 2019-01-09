use crate::ray::Ray;
use crate::scene::SceneObject;

use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64) -> Self {
        Self { origin, radius }
    }
}

impl SceneObject for Sphere {
    // This ray must be in the same basis the sphere's origin is in
    fn collision(&self, ray: &Ray) -> Option<Vector3<f64>> {
        let norm = ray.origin() - self.origin;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * norm.dot(ray.direction());
        let c = norm.dot(norm) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;
        if disc > 0.0 {
            let solution1 = (-b + (b * b - 4.0 * a * c).sqrt()) / a;
            let solution2 = (-b - (b * b - 4.0 * a * c).sqrt()) / a;

            // We only want collisions that occur in the positive direction of the ray
            if solution1 > 0.0 || solution2 > 0.0 {
                return Some(Vector3::new(0.0, 0.0, 0.0));
            }
        }

        None
    }
}
