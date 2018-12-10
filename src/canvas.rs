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

    // Render and then write to flie
    pub fn capture<'a>(&self, scene: &'a Scene, out_file_path: String) {
        let mut image_buffer = image::RgbImage::new(self.x_size as u32, self.y_size as u32);

        for y_pixel in (0..self.y_size).rev() {
            for x_pixel in 0..self.x_size {
                let ray_source = self.location;
                let ray_direction = (self.camera_distance * self.direction)
                    + Vector3::new(
                        x_pixel as f64 - self.x_size as f64 / 2.0,
                        y_pixel as f64 - self.y_size as f64 / 2.0,
                        0.0,
                    );

                let current_ray = Ray::new(ray_source, ray_direction);
                let colour = Self::colour_ray(&current_ray);
                image_buffer.put_pixel(
                    x_pixel as u32,
                    y_pixel as u32,
                    image::Rgb([colour.x, colour.y, colour.z]),
                );
            }
        }

        // Create an 8bit pixel of type Luma and value i
        // and assign in to the pixel at position (x, y)

        // Save the image as “fractal.png”, the format is deduced from the path
        image_buffer.save(format!("{}.png", out_file_path)).unwrap();
    }

    fn colour_ray(ray: &Ray) -> Vector3<u8> {
        let t = (ray.direction().normalize().y + 1.0) / 2.0;
        let colour = (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.7, 0.7, 1.0);
        Vector3::new((colour.x * 255.0) as u8, (colour.y * 255.0) as u8, (colour.z * 255.0) as u8)
    }
}
