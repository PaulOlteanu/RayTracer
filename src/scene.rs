pub mod scene_objects;

pub trait SceneObject {}

pub struct Scene {
    scene_objects: Vec<Box<dyn SceneObject>>,
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
}
