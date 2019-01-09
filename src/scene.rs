pub mod scene_objects;

use crate::ray::Ray;
use cgmath::{InnerSpace, Vector3};

pub trait SceneObject: std::fmt::Debug {
    fn collision(&self, ray: &Ray) -> Option<Vector3<f64>>;
}

pub struct Scene {
    scene_objects: Vec<Box<SceneObject>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            scene_objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, obj: impl SceneObject + 'static) {
        self.scene_objects.push(Box::new(obj));
    }

    pub fn get_collisions(&self, ray: &Ray) -> Vec<&SceneObject> {
        let mut result = Vec::new();

        for object in self.scene_objects.iter() {
            if object.collision(ray).is_some() {
                result.push(&**object);
            }
        }

        result
    }
}
