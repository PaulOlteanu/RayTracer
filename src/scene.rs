pub mod scene_objects;

use crate::ray::Ray;

pub trait SceneObject: std::fmt::Debug {
   fn collided(&self, ray: &Ray) -> bool;
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
            if object.collided(ray) {
                result.push(&**object);
            }
        }

        result
    }
}
