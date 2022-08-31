use std::fs::File;

use raytracer::canvas::{Canvas, Color};
use raytracer::lighting::{Intersection, Material, PointLight, Ray};
use raytracer::matrix::transformation;
use raytracer::shape::Sphere;
use raytracer::tuple::Tuple;

fn main() {
    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 200;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    // Sphere
    let mut sphere_material = Material::default();
    sphere_material.color = Color::new(1.0, 0.2, 1.0);
    let sphere_transformation = transformation::rotation_z(std::f64::consts::FRAC_PI_4)
        * transformation::scaling(1.0, 0.5, 1.0);
    let sphere = Sphere::new(sphere_transformation, sphere_material);

    // Light
    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas_pixels {
            // Tanto `world_x` como `world_y` sirven para pasar las dimensiones del canvas a puntos
            // sobre el "wall" que tengo en mi world space.
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = sphere.intersect(ray);

            if let Some(hit) = Intersection::hit(xs) {
                let point = ray.position(hit.t);
                let normal = sphere.normal_at(point);
                let eye = -ray.direction;

                let color = hit.object.material.lighting(light, point, eye, normal);

                canvas.write_pixel(x, y, color).unwrap();
            }
        }
    }

    let mut file = File::create("image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}
