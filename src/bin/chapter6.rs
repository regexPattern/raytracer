use std::fs::File;

use raytracer::canvas::{Canvas, Color};
use raytracer::intersection::{Intersection, Material, PointLight, Ray, Sphere};
use raytracer::matrix::transformation::Transformation;
use raytracer::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 200;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();

    shape.material = Material::default();
    shape.material.color = Color::new(1.0, 0.2, 1.0);

    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = ray.intersect(shape);

            if let Some(hit) = Intersection::hit(&xs) {
                let point = ray.position(hit.t);
                let normal = shape.normal_at(point);
                let eye = -ray.direction;

                let color = hit.object.material.lighting(light, point, eye, normal);

                canvas.write_pixel(x, y, color);
            }
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}