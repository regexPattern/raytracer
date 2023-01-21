#![allow(unused, dead_code)]

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    obj::Parser,
    shape::{BaseShape, Cylinder, Group, Shape, Triangle},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::WHITE,
    };

    let triangle = Shape::Triangle(
        Triangle::try_new(
            Point::new(1.0, 0.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(0.0, 2.0, 0.),
        )
        .unwrap(),
    );

    let obj_file = std::fs::read_to_string("teapot.obj").unwrap();
    let parser = Parser::parse(&obj_file).unwrap();

    // TODO: Cambiar estos a IntoIterator.
    let world = World {
        objects: vec![Shape::Group(parser.group)],
        lights: vec![light],
    };

    let camera = Camera::try_new(9 * 40, 16 * 40, std::f64::consts::FRAC_PI_3)
        .unwrap()
        .with_transform(
            Transform::try_view(
                Point::new(0.0, 3.0, 5.0),
                Point::new(0.0, 0.0, 0.0),
                Vector::new(0.0, 1.0, 0.0),
            )
            .unwrap(),
        );

    let image = camera
        .render(&world, raytracer::camera::RenderProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
