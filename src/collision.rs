use crate::shading::ShadingData;

use cgmath::Vector3;

pub struct Collision<'a> {
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub shading_data: &'a ShadingData,
}
