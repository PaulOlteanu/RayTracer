use cgmath::{InnerSpace, Vector3};

use crate::ray::Ray;
use crate::scene::Scene;

pub struct Canvas {
    x_size: u32,
    y_size: u32,
    camera_distance: f64,
    location: Vector3<f64>,
    direction: Vector3<f64>, // This is a unit vector
}

impl Canvas {
    pub fn new(
        x_size: u32,
        y_size: u32,
        camera_distance: f64,
        location: Vector3<f64>,
        direction: Vector3<f64>,
    ) -> Self {
        Self {
            x_size,
            y_size,
            camera_distance,
            location,
            direction,
        }
    }

    // Render and then write to file
    pub fn capture<'a>(&self, scene: &'a Scene, out_file_path: String) {
        let mut image_buffer = image::RgbImage::new(self.x_size as u32, self.y_size as u32);

        for (x_pixel, y_pixel, pixel) in image_buffer.enumerate_pixels_mut() {
            // The image buffer uses y in the top left but we want it in the bottom left so flip it
            let y_pixel = self.y_size as f64 - y_pixel as f64;

            let ray_source = self.location;

            // The direction ray is multipled to the "screen", then is shifted up/down/left/right to the current pixel
            let ray_direction = (self.camera_distance * self.direction)
                + Vector3::new(
                    x_pixel as f64 - self.x_size as f64 / 2.0,
                    y_pixel as f64 - self.y_size as f64 / 2.0,
                    0.0,
                );

            // In terms of scene's basis
            let collision_ray = Ray::new(ray_source, ray_direction - ray_source);

            let colour = Self::colour_ray(&collision_ray, scene);
            *pixel = image::Rgb([colour.x, colour.y, colour.z]);
        }

        image_buffer.save(format!("{}.png", out_file_path)).unwrap();
    }

    fn colour_ray(ray: &Ray, scene: &Scene) -> Vector3<u8> {
        // Check for collisions using a ray that's in terms of the scene's coordinates and basis
        let collisions = scene.get_collisions(&ray);

        // Find the first thing collided with
        if collisions.len() > 0 {
            let mut first_collision = collisions.first().unwrap();
            for collision in collisions.iter() {
                if (ray.origin() - collision.point).magnitude() < (ray.origin() - first_collision.point).magnitude() {
                    first_collision = collision;
                }
            }

            let colour = 0.5 * Vector3::new(first_collision.normal.x + 1.0, first_collision.normal.y + 1.0, first_collision.normal.z + 1.0);

            return Vector3::new(
                (colour.x * 255.0) as u8,
                (colour.y * 255.0) as u8,
                (colour.z * 255.0) as u8,
            )
        }

        let t = (ray.direction().normalize().y + 1.0) / 2.0;
        let colour = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
        Vector3::new(
            (colour.x * 255.0) as u8,
            (colour.y * 255.0) as u8,
            (colour.z * 255.0) as u8,
        )
    }
}
