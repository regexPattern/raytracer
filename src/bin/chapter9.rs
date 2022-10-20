use std::fs::File;
use std::io::prelude::*;

use raytracer::camera::Camera;
use raytracer::color::{self, Color};
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::matrix::Matrix;
use raytracer::shape::{Figure, Plane, Shapes, Sphere};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

fn main() {
    let middle = Shapes::Sphere(Sphere(Figure {
        transform: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color {
                red: 0.1,
                green: 1.0,
                blue: 0.5,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    }));

    let right = Shapes::Sphere(Sphere(Figure {
        transform: Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            color: Color {
                red: 0.5,
                green: 1.0,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    }));

    let left = Shapes::Sphere(Sphere(Figure {
        transform: Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.8,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    }));

    let plane = Shapes::Plane(Plane::default());

    let objects = vec![middle, right, left, plane];
    let lights = vec![PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::WHITE,
    }];

    let world = World { objects, lights };

    let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(5.0, 3.0, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);
    let ppm = image.to_ppm();

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
