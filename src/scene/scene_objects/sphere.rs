use crate::collision::Collision;
use crate::ray::Ray;
use crate::scene::SceneObject;
use crate::shading::Shader;

use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
    shader: Box<Shader>,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64, shader: Box<Shader>) -> Self {
        Self {
            origin,
            radius,
            shader,
        }
    }
}

impl SceneObject for Sphere {
    // This ray must be in the same basis the sphere's origin is in
    fn collision(&self, ray: &Ray) -> Option<Collision> {
        let oc = ray.origin() - self.origin;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;
        if disc > 0.0 {
            let solution = (-b - disc.sqrt()) / (2.0 * a);
            if solution > 0.0 {
                let point = ray.origin() + solution * ray.direction();
                let normal = (point - self.origin).normalize();

                return Some(Collision {
                    point,
                    normal,
                    shading_data: self.get_shading_data(),
                });
            }

            let solution = (-b + disc.sqrt()) / (2.0 * a);
            if solution > 0.0 {
                let point = ray.origin() + solution * ray.direction();
                let normal = (point - self.origin).normalize();

                return Some(Collision {
                    point,
                    normal,
                    shading_data: self.get_shading_data(),
                });
            }
        }

        None
    }

    fn get_shading_data(&self) -> &Box<Shader> {
        &self.shader
    }
}
