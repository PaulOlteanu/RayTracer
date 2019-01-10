pub mod scene_objects;

use crate::collision::Collision;
use crate::ray::Ray;
use crate::shading::ShadingData;

pub trait SceneObject: std::fmt::Debug {
    fn collision(&self, ray: &Ray) -> Option<Collision>;
    fn get_shading_data(&self) -> &ShadingData;
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

    pub fn get_collisions(&self, ray: &Ray) -> Vec<Collision> {
        let mut collisions = Vec::new();

        for object in self.scene_objects.iter() {
            if let Some(collision) = object.collision(ray) {
                collisions.push(collision);
            }
        }

        collisions
    }
}
