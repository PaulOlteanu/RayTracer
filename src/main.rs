use cgmath::Vector3;

mod canvas;
mod collision;
mod ray;
mod scene;
mod shading;

use self::canvas::Canvas;
use self::scene::{scene_objects::Sphere, Scene};
use self::shading::ShadingData;

const CANVAS_WIDTH: u32 = 200;
const CANVAS_HEIGHT: u32 = 100;
const CAMERA_DISTANCE: f64 = 100.0;

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

    let sphere1_origin = Vector3::new(0.0, 0.0, -200.0);
    let sphere1_radius = 50.0;
    let sphere1_shading = ShadingData {
        colour: Vector3::new(0, 0, 0),
    };
    let sphere1 = Sphere::new(sphere1_origin, sphere1_radius, sphere1_shading);

    let sphere2_origin = Vector3::new(0.0, -10050.0, -200.0);
    let sphere2_radius = 10000.0;
    let sphere2_shading = ShadingData {
        colour: Vector3::new(0, 0, 0),
    };
    let sphere2 = Sphere::new(sphere2_origin, sphere2_radius, sphere2_shading);

    scene.add_object(sphere1);
    scene.add_object(sphere2);

    canvas.capture(&scene, String::from("asdf"));
}
