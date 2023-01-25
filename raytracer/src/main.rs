#![allow(unused, dead_code)]

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    shape::{Cylinder, Group, Shape, ShapeProps, Triangle},
    transform::Transform,
    tuple::{Point, Vector},
    obj_model::OBJModel,
    world::World,
};

fn main() {
    let light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::WHITE,
    };

    let obj_file = std::fs::read_to_string("teapot.obj").unwrap();
    let model = OBJModel::import(&obj_file).unwrap();
    let mut group = Group::from(model);
    group.divide(300);

    let world = World {
        objects: vec![Shape::Group(group)],
        lights: vec![light],
    };

    let camera = Camera::try_new(
        360,
        640,
        std::f64::consts::FRAC_PI_3,
        Transform::try_view(
            Point::new(0.0, 3.0, 5.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    )
    .unwrap();

    let image = camera
        .render(&world, raytracer::camera::RenderProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
