use std::error::Error;
use std::fs::File;

use raytracer::canvas::Canvas;
use raytracer::intersection::Intersection;
use raytracer::ray::Ray;
use raytracer::shape::{Intersectable, Sphere};
use raytracer::tuple::{Color, Point};

fn main() -> Result<(), Box<dyn Error>> {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let shape = Sphere::default();

    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            let xs = shape.intersect(ray);

            if Intersection::hit(xs).is_some() {
                canvas.write_pixel(x, y, color)?;
            }
        }
    }

    let mut file = File::create("image.ppm")?;
    canvas.to_ppm(&mut file)?;
    Ok(())
}
