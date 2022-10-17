#![allow(clippy::unwrap_used)]

use std::fs::File;
use std::io::Write;

use raytracer::camera::Camera;
use raytracer::color::{self, Color};
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::matrix::Matrix;
use raytracer::sphere::Sphere;
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

fn main() {
    let floor = Sphere {
        transform: Matrix::scaling(10.0, 0.01, 10.0),
        material: Material {
            color: color::WHITE,
            specular: 0.0,
            ..Material::default()
        },
    };

    let left_wall = Sphere {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0),
        material: floor.material.clone(),
    };

    let right_wall = Sphere {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2)
            * Matrix::scaling(10.0, 0.01, 10.0),
        material: floor.material.clone(),
    };

    let middle = Sphere {
        transform: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color {
                red: 0.1,
                green: 1.0,
                blue: 0.5,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    };

    let right = Sphere {
        transform: Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            color: Color {
                red: 0.5,
                green: 1.0,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    };

    let left = Sphere {
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
    };

    let left_light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::WHITE,
    };

    let right_light = PointLight {
        position: Point::new(10.0, 10.0, -5.0),
        intensity: Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        },
    };

    let objects = vec![floor, left_wall, right_wall, middle, right, left];
    let lights = vec![left_light, right_light];

    let world = World { objects, lights };

    let mut camera = Camera::new(3840, 2160, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(0.0, 3.0, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);
    let ppm = image.to_ppm();

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
