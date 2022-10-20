use std::fs::File;
use std::io::prelude::*;

use raytracer::camera::Camera;
use raytracer::color::{Color, RGBColor};
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::matrix::Matrix;
use raytracer::shape::{Shape, Shapes};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

fn main() {
    let middle = Shapes::Sphere(Shape {
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
    });

    let right = Shapes::Sphere(Shape {
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
    });

    let left = Shapes::Sphere(Shape {
        transform: Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.8,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    });

    let floor = Shapes::Plane(Shape::default());

    let left_wall = Shapes::Plane(Shape {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    });

    let right_wall = Shapes::Plane(Shape {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    });

    let left_light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        // rgb(170, 120, 120)
        intensity: Color::from(RGBColor {
            red: 170,
            green: 120,
            blue: 120,
        }),
    };

    let right_light = PointLight {
        position: Point::new(10.0, 10.0, -10.0),
        // rgb(120, 120, 170)
        intensity: Color::from(RGBColor {
            red: 120,
            green: 120,
            blue: 170,
        }),
    };

    let objects = vec![middle, right, left, floor, left_wall, right_wall];
    let lights = vec![left_light, right_light];

    let world = World { objects, lights };

    let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    // let mut camera = Camera::new(1920, 1080, std::f64::consts::FRAC_PI_3);
    // let mut camera = Camera::new(3840, 2160, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(5.0, 3.0, -10.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);
    let ppm = image.to_ppm();

    let mut file = File::create("image.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
