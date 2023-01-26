#![allow(unused, dead_code)]

use std::str::FromStr;

use raytracer::{
    camera::{Camera, RenderProgress},
    color,
    light::PointLight,
    obj_model::OBJModel,
    shape::{Group, Shape},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    let content = std::fs::read_to_string("dragon.obj").unwrap();
    let model = OBJModel::from_str(&content).unwrap();
    let mut group = Group::from(model);

    group.divide(500);

    let group = Shape::Group(group);

    let light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![group],
        lights: vec![light],
    };

    let camera = Camera::try_new(
        500,
        500,
        std::f64::consts::FRAC_PI_3,
        Transform::translation(0.0, 0.0, -20.0),
    )
    .unwrap();

    let image = camera.render(&world, RenderProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
