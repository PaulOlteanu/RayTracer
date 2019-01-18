use cgmath::{InnerSpace, Vector3};
use rand::prelude::*;

use crate::ray::Ray;
use crate::scene::Scene;
use crate::util;

pub struct Canvas {
    x_size: u32,
    y_size: u32,
    camera_distance: f64,
    location: Vector3<f64>,
    direction: Vector3<f64>, // This is a unit vector
}

// Pixels are 2 units wide so that 1 unit is in the centre of them
// This is for future reference but probably belongs somewhere else
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
    pub fn capture<'a>(&self, scene: &'a Scene, iterations: u64, out_file_path: String) {
        let mut image_buffer = image::RgbImage::new(self.x_size as u32, self.y_size as u32);

        let mut rng = rand::thread_rng();

        for (x_pixel, y_pixel, pixel) in image_buffer.enumerate_pixels_mut() {
            // The image buffer uses y in the top left but we want it in the bottom left so flip it
            let x_pixel = x_pixel + 1;
            let y_pixel = self.y_size - y_pixel;

            let x_coordinate = 2.0 * x_pixel as f64 - self.x_size as f64 - 1.0;
            let y_coordinate = 2.0 * y_pixel as f64 - self.y_size as f64 - 1.0;

            // println!("COORDINATE: {:?}, {:?}", x_coordinate, y_coordinate);

            let mut colour = Vector3::new(0, 0, 0);

            for _iteration in 0..iterations {
                let ray_source = self.location;

                let x_offset: f64 = rng.gen::<f64>() * 2.0 - 1.0;
                let y_offset: f64 = rng.gen::<f64>() * 2.0 - 1.0;

                // The direction ray is multipled to the "screen", then is shifted up/down/left/right to the current pixel
                // TODO: This needs to be changed to not be z 0
                let ray_direction = ((self.camera_distance * self.direction)
                    + Vector3::new(x_coordinate + x_offset, y_coordinate + y_offset, 0.0)
                    - ray_source)
                    .normalize();

                let collision_ray = Ray::new(ray_source, ray_direction);
                // TODO: This is terrible
                let col = Self::colour_ray(&collision_ray, scene, 0, 50);
                colour += Vector3::new(
                    (col.x.sqrt() * 255.0) as u64,
                    (col.y.sqrt() * 255.0) as u64,
                    (col.z.sqrt() * 255.0) as u64,
                );
                // colour += Vector3::new(col.x as u64, col.y as u64, col.z as u64);
            }
            colour /= iterations;

            *pixel = image::Rgb([colour.x as u8, colour.y as u8, colour.z as u8]);
        }

        image_buffer.save(format!("{}.png", out_file_path)).unwrap();
    }

    fn colour_ray(
        ray: &Ray,
        scene: &Scene,
        current_reflections: u32,
        max_reflections: u32,
    ) -> Vector3<f64> {
        if current_reflections == max_reflections {
            return Vector3::new(1.0, 1.0, 1.0);
        }
        // Check for collisions using a ray that's in terms of the scene's coordinates and basis
        let collisions = scene.get_collisions(&ray);

        // Find the first thing collided with
        let colour = if collisions.len() > 0 {
            let mut first_collision = collisions.first().unwrap();
            for collision in collisions.iter() {
                if (ray.origin() - collision.point).magnitude()
                    < (ray.origin() - first_collision.point).magnitude()
                {
                    first_collision = collision;
                }
            }

            if let Some((attenuation, scatter)) = first_collision.shading_data.scatter(ray, first_collision) {
                let next_colour = Self::colour_ray(&scatter, scene, current_reflections + 1, max_reflections);
                Vector3::new(attenuation.x * next_colour.x, attenuation.y * next_colour.y, attenuation.z * next_colour.z)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            }
        } else {
            let t = (ray.direction().normalize().y + 1.0) / 2.0;
            (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
        };

        colour
    }
}
