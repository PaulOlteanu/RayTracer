use cgmath::{InnerSpace, Vector3};
use rand::prelude::*;

pub fn point_in_sphere(radius: f64) -> Vector3<f64> {
    let mut rng = rand::thread_rng();

    let direction: Vector3<f64> = ((2.0
        * Vector3::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()))
        - Vector3::new(1.0, 1.0, 1.0))
    .normalize();
    let mag: f64 = rng.gen_range(1E-2, radius);

    mag * direction
}
