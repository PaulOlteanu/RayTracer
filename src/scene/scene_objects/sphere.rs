use crate::scene::SceneObject;
use crate::ray::Ray;

use cgmath::{Vector3, InnerSpace};


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

    // This ray needs to be in the same basis the sphere's origin is in
    fn collided(&self, ray: &Ray) -> bool {
        // println!("RAY: {:?}", ray);
        let norm = ray.origin() - self.origin;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * norm.dot(ray.direction());
        let c = norm.dot(norm) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;
        return disc > 0.0;
    }
}
