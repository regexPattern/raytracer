use std::fs::File;

use raytracer::canvas::{Canvas, Color};
use raytracer::intersection::Intersection;
use raytracer::ray::Ray;
use raytracer::shape::Sphere;
use raytracer::transformation;
use raytracer::tuple::Tuple;

fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);

    let wall_z = 10.0;
    let wall_size = 6.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let shape_transform = transformation::translation(25.0 * pixel_size, 0.0, 0.0);
    let shape = Sphere::from(shape_transform);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::vector(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());

            let xs = shape.intersect(ray);

            if Intersection::hit(xs).is_some() {
                canvas.write_pixel(x, y, color).unwrap();
            }
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}
