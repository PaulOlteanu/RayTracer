use crate::shading::Shader;

use cgmath::Vector3;

#[derive(Debug)]
pub struct Collision<'a> {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub shading_data: &'a Box<Shader>,
}
