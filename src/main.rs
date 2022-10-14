use std::fs::File;
use std::io::Write;

use raytracer::canvas::Canvas;
use raytracer::color::{self, Color};
use raytracer::intersection::Intersection;
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::tuple::Point;

fn main() {
    let s = Sphere {
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.2,
                blue: 1.0,
            },
            ..Material::default()
        },
        ..Sphere::default()
    };

    let light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::WHITE,
    };

    run(s, light);
}

fn run(s: Sphere, light: PointLight) {
    let ray_origin = Point::new(0.0, 0.0, -5.0);

    let wall_size = 7.0;
    let wall_z = 10.0;

    let canvas_pixels = 100;
    let pixel_size = wall_size / f64::from(canvas_pixels);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * f64::from(y);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * f64::from(x);

            let position = Point::new(world_x, world_y, wall_z);

            let r = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).normalize(),
            };
            let xs = s.intersect(&r);

            if let Some(i) = Intersection::hit(xs) {
                let point = r.position(i.t);
                let normal = s.normal_at(point);
                let eye = -r.direction;

                let color = i.object.material.lighting(&light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let ppm = canvas.to_ppm();
    let mut file = File::create("image.ppm").unwrap();
    let _ = file.write(ppm.as_bytes()).unwrap();
}
