use std::error::Error;
use std::fs::File;

use raytracer::camera::Camera;
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::shape::{Intersectable, Shape, Sphere};
use raytracer::transformation;
use raytracer::tuple::{Color, Point, Vector};
use raytracer::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let floor = Shape::Sphere(Sphere {
        transform: transformation::scaling(10.0, 0.01, 10.0),
        material: Material {
            color: Color::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Material::default()
        },
    });

    let left_wall = Shape::Sphere(Sphere {
        material: floor.material(),
        transform: transformation::translation(0.0, 0.0, 5.0)
            * transformation::rotation_y(-std::f64::consts::FRAC_PI_4)
            * transformation::rotation_x(std::f64::consts::FRAC_PI_2)
            * floor.transform(),
    });

    let right_wall = Shape::Sphere(Sphere {
        material: floor.material(),
        transform: transformation::translation(0.0, 0.0, 5.0)
            * transformation::rotation_y(std::f64::consts::FRAC_PI_4)
            * transformation::rotation_x(std::f64::consts::FRAC_PI_2),
    });

    let middle = Shape::Sphere(Sphere {
        transform: transformation::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Material::default()
        },
    });

    let right = Shape::Sphere(Sphere {
        transform: transformation::translation(1.5, 0.5, -0.5)
            * transformation::scaling(0.5, 0.5, 0.5),
        material: Material {
            color: Color::new(0.5, 1.0, 0.1),
            ..middle.material()
        },
    });

    let left = Shape::Sphere(Sphere {
        transform: transformation::translation(-1.5, 0.33, -0.75)
            * transformation::scaling(0.33, 0.33, 0.33),
        material: Material {
            color: Color::new(1.0, 0.8, 0.1),
            ..middle.material()
        },
    });

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());

    let world = World::new(
        vec![floor, left_wall, right_wall, middle, right, left],
        light,
    );

    let mut camera = Camera::new(300, 150, std::f64::consts::FRAC_PI_3);
    camera.transform = transformation::view(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world)?;

    let mut file = File::create("image.ppm")?;
    canvas.to_ppm(&mut file)?;
    Ok(())
}
