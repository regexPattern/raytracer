#![allow(unused_variables, unused_imports)]

use raytracer::{
    camera::{Camera, RenderProgress},
    color::{self, Color},
    light::PointLight,
    material::{self, Material},
    pattern::{Pattern3D, Pattern3D},
    shape::{Object, Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let sky = Shape::Plane(Plane(Object {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::LIGHT_SKY_BLUE),
            shininess: 1000.0,
            specular: 0.0,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 0.0, 100.0)
            * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    }));

    let floor = Shape::Plane(Plane(Object {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.6078,
                green: 0.4627,
                blue: 0.3255,
            }),
            ambient: -1.0,
            diffuse: -1.0,
            index_of_refraction: -1.0,
            reflectivity: -1.0,
            shininess: -1.0,
            specular: -1.0,
            transparency: -1.0,
        },
        ..Default::default()
    }));

    let sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1,
            }),
            reflectivity: 0.9,
            transparency: 0.9,
            shininess: 600.0,
            ambient: 0.01,
            index_of_refraction: material::consts::GLASS_INDEX_OF_REFRACTION,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 1.0, 0.0),
    }));

    let checkered_sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            pattern: Pattern3D::Checker(Pattern3D {
                a: color::consts::WHITE,
                b: color::consts::BLACK,
                transform: Transform::try_scaling(0.25, 0.25, 0.25).unwrap(),
            }),
            ..Default::default()
        },
        transform: Transform::translation(0.0, 1.0, 0.0),
    }));

    let light = PointLight {
        position: Point::new(20.0, 50.0, -20.0),
        intensity: color::consts::WHITE,
    };

    let objects = vec![floor, checkered_sphere, sky];
    let lights = vec![light];

    let world = World { objects, lights };

    let mut camera = Camera::try_new(720, 720, std::f64::consts::FRAC_PI_3).unwrap();
    camera.transform = Transform::try_view(
        Point::new(0.0, 3.0, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    )
    .unwrap();

    let image = camera.render(&world, RenderProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
