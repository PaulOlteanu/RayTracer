use cgmath::Vector3;

mod canvas;
mod ray;
mod scene;
mod shading;

use self::canvas::Canvas;
use self::scene::{scene_objects::Sphere, Scene};

const CANVAS_WIDTH: u32 = 200;
const CANVAS_HEIGHT: u32 = 100;
const CAMERA_DISTANCE: f64 = 20.0;

fn main() {
    let _x = Vector3::new(1.0, 0.0, 0.0);
    let _y = Vector3::new(0.0, 1.0, 0.0);
    let z = Vector3::new(0.0, 0.0, 1.0);

    let camera_origin = Vector3::new(0.0, 0.0, 0.0);
    let camera_direction = -z; // Pointing to the negative z axis

    let mut scene = Scene::new();

    let canvas = Canvas::new(
        CANVAS_WIDTH,
        CANVAS_HEIGHT,
        CAMERA_DISTANCE,
        camera_origin,
        camera_direction,
    );

    let sphere1_origin = Vector3::new(0.0, -20.0, -50.0);
    let sphere1_radius = 20.0;
    let sphere1 = Sphere::new(sphere1_origin, sphere1_radius);

    scene.add_object(sphere1);

    canvas.capture(&scene, String::from("asdf"));
}
