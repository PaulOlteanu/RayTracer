use cgmath::Vector3;

#[derive(Debug)]
pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn direction(&self) -> Vector3<f64> {
        self.direction
    }
}
