#![allow(unused, dead_code)]

use std::{num::NonZeroUsize, str::FromStr};

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    obj_model::OBJModel,
    scene::SceneProgress,
    shape::{Cube, Cylinder, Group, Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn hexagon_corner() -> Shape {
    Shape::Sphere(Sphere::new(
        Default::default(),
        Transform::translation(0.0, 0.0, -1.0) * Transform::scaling(0.25, 0.25, 0.25).unwrap(),
    ))
}

fn hexagon_edge() -> Shape {
    Shape::Cylinder(Cylinder::new(
        Default::default(),
        Transform::translation(0.0, 0.0, -1.0)
            * Transform::rotation_y(-std::f64::consts::FRAC_PI_6)
            * Transform::rotation_z(-std::f64::consts::FRAC_PI_2)
            * Transform::scaling(0.25, 1.0, 0.25).unwrap(),
        0.0,
        1.0,
        false,
    ))
}

fn hexagon_side(transform: Transform) -> Group {
    Group::new([hexagon_corner(), hexagon_edge()], transform)
}

fn hexagon() -> Shape {
    let mut group = Group::default();

    for i in 0..6 {
        let side = hexagon_side(Transform::rotation_y(
            f64::from(i) * std::f64::consts::FRAC_PI_3,
        ));
        group.push(Shape::Group(side));
    }

    Shape::Group(group)
}

fn main() {
    // let content = std::fs::read_to_string("dragon.obj").unwrap();
    // let model = OBJModel::from_str(&content).unwrap();
    // let mut group = Group::from(model);

    // group.divide(500);

    // let group = Shape::Group(group);

    let plane = Shape::Plane(Plane::default());

    let sphere = Shape::Sphere(Sphere::new(
        Default::default(),
        Transform::translation(0.0, 0.0, 0.0),
    ));
    let cylinder = Shape::Cylinder(Cylinder::new(
        Default::default(),
        Default::default(),
        0.0,
        3.0,
        false,
    ));

    let group0 = Group::new(
        [cylinder],
        Transform::rotation_z(0.0),
    );

    let cube = Shape::Cube(Cube::default());

    let group1 = Group::new([cube], Transform::translation(0.0, 0.0, 0.0));

    let group2 = Group::new(
        [Shape::Group(group0), Shape::Group(group1)],
        Transform::scaling(2.0, 1.0, 1.0).unwrap(),
    );

    let light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![Shape::Group(group2)],
        lights: vec![light],
    };

    let camera = Camera::new(
        NonZeroUsize::new(500).unwrap(),
        NonZeroUsize::new(500).unwrap(),
        std::f64::consts::FRAC_PI_3,
        Transform::view(
            Point::new(5.0, 5.0, -5.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    )
    .unwrap();

    let image = camera.render(&world, SceneProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
