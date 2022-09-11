use std::error::Error;
use std::fs::File;

use raytracer::camera::Camera;
use raytracer::light::PointLight;
use raytracer::material::Material;
use raytracer::shape::{Intersectable, Plane, Shape, Sphere};
use raytracer::transformation;
use raytracer::tuple::{Color, Point, Vector};
use raytracer::world::World;

fn main() -> Result<(), Box<dyn Error>> {
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

    let plane = Shape::Plane(Plane::default());

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::white());
    let objects = vec![middle, right, left, plane];

    let world = World::new(objects, light);

    let mut camera = Camera::new(300, 150, std::f64::consts::FRAC_PI_2);
    camera.transform = transformation::view(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    if let Ok(canvas) = camera.render(&world) {
        let mut file = File::create("image.ppm")?;
        canvas.to_ppm(&mut file)?;
    };

    Ok(())
}
