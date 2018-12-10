use crate::scene::SceneObject;
use cgmath::Vector3;

pub struct Sphere {
    origin: Vector3<i32>,
    radius: u32,
}

impl Sphere {
    pub fn new(origin: Vector3<i32>, radius: u32) -> Self {
        Self { origin, radius }
    }
}

impl SceneObject for Sphere {}
