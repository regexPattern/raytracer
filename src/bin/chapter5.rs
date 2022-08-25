use std::fs::File;

use raytracer::canvas::{Canvas, Color};
use raytracer::intersection::{Intersection, Ray, Sphere};
use raytracer::matrix::transformation::Transformation;
use raytracer::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();
    shape.transform = Transformation::rotation_z(std::f64::consts::FRAC_PI_4)
        * Transformation::scaling(1.0, 0.5, 1.0);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = ray.intersect(shape);

            if Intersection::hit(&xs).is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}