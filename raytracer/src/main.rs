#![allow(unused, dead_code)]

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    material::Material,
    object::{Cylinder, Group, Object, Plane, Sphere},
    pattern::{Pattern, Schema},
    transform::Transform,
    tuple::{Point, Vector},
    world::{self, World},
};

fn main() {
    let sky = Object::Plane(Plane {
        material: Material {
            pattern: Pattern::Solid(color::consts::LIGHT_SKY_BLUE),
            ..Default::default()
        },
        transform: Transform::translation(-40.0, 0.0, 0.0)
            * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
    });

    let floor = Object::Plane(Plane {
        material: Material {
            shininess: 200.0,
            pattern: Pattern::Solid(color::consts::DIRT),
            ..Default::default()
        },
        transform: Transform::translation(0.0, 0.0, 0.0),
    });

    let s1 = Object::Sphere(Sphere {
        material: Material {
            pattern: Pattern::Checker(Schema {
                a: color::consts::LIGHT_SKY_BLUE,
                b: color::consts::BLUE,
                transform: Default::default(),
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    let c1 = Object::Cylinder(Cylinder {
        minimum: 0.0,
        maximum: 1.0,
        closed: true,
        material: Material {
            pattern: Pattern::Checker(Schema {
                a: color::consts::WHITE,
                b: color::consts::RED,
                transform: Default::default(),
            }),
            ..Default::default()
        },
        transform: Transform::translation(0.0, 0.0, 3.0) * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    });

    let mut group = Group {
        children: vec![],
        transform: Transform::translation(0.0, 1.0, 0.0),
    };

    group.add_child(s1);
    group.add_child(c1);

    let group = Object::Group(group);

    let light = PointLight {
        position: Point::new(40.0, 40.0, 40.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![floor, sky, group],
        lights: vec![light],
    };

    let mut camera = Camera::try_new(480, 270, std::f64::consts::FRAC_PI_3).unwrap();
    camera.transform = Transform::try_view(
        Point::new(8.0, 3.0, 20.0),
        Point::new(0.0, 2.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    )
    .unwrap();

    let image = camera
        .render(&world, raytracer::camera::RenderProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
