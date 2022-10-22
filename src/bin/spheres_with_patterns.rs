#![allow(clippy::unwrap_used)]
#![allow(dead_code)]

use std::fs::File;
use std::io::prelude::*;

use raytracer::camera::Camera;
use raytracer::color::{self, Color, RGB};
use raytracer::light::PointLight;
use raytracer::material::{Material, Texture};
use raytracer::matrix::Matrix;
use raytracer::pattern::Striped;
use raytracer::shape::{Shape, ShapeProps};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

const RES_HD: (u32, u32) = (1280, 720);
const RES_FULL_HD: (u32, u32) = (1920, 1080);
const RES_4K: (u32, u32) = (3840, 2160);

const RESOLUTION: (u32, u32) = RES_4K;

fn main() {
    let middle = Shape::Sphere(ShapeProps {
        transform: Matrix::translation(-0.5, 1.0, 0.5),
        material: Material {
            texture: Texture::Pattern(Striped {
                a: color::WHITE,
                b: color::RED,
                transform: Matrix::rotation_y(std::f64::consts::PI)
                    * Matrix::scaling(0.5, 0.5, 0.5),
            }),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    });

    let right = Shape::Sphere(ShapeProps {
        transform: Matrix::translation(1.5, 0.5, -0.5) * Matrix::scaling(0.5, 0.5, 0.5),
        material: Material {
            texture: Texture::Color(Color {
                red: 0.5,
                green: 1.0,
                blue: 0.1,
            }),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    });

    let left = Shape::Sphere(ShapeProps {
        transform: Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scaling(0.33, 0.33, 0.33),
        material: Material {
            texture: Texture::Color(Color {
                red: 1.0,
                green: 0.8,
                blue: 0.1,
            }),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    });

    let floor = Shape::Plane(ShapeProps::default());

    let left_wall = Shape::Plane(ShapeProps {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(-std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    });

    let right_wall = Shape::Plane(ShapeProps {
        transform: Matrix::translation(0.0, 0.0, 5.0)
            * Matrix::rotation_y(std::f64::consts::FRAC_PI_4)
            * Matrix::rotation_x(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    });

    let blue_light = PointLight {
        position: Point::new(10.0, 10.0, -10.0),
        // rgb(130, 130, 180)
        intensity: Color::from(RGB {
            red: 130,
            green: 130,
            blue: 180,
        }),
    };

    let red_light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        // rgb(180, 130, 130)
        intensity: Color::from(RGB {
            red: 180,
            green: 130,
            blue: 130,
        }),
    };

    let objects = vec![middle, right, left, floor, right_wall, left_wall];
    let lights = vec![blue_light, red_light];

    let world = World { objects, lights };

    let mut camera = Camera::new(RESOLUTION.0, RESOLUTION.1, std::f64::consts::FRAC_PI_3);
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
