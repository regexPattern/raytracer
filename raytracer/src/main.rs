#![allow(unused, dead_code)]

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
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
            Default::default(),
            Point::new(1.0, 0.0, 0.0),
            Point::new(-1.0, 0.0, 0.0),
            Point::new(0.0, 2.0, 0.),
        )
        .unwrap(),
    );

    // TODO: Cambiar estos a IntoIterator.
    let world = World {
        objects: vec![triangle],
        lights: vec![light],
    };

    let camera = Camera::try_new(200, 200, std::f64::consts::FRAC_PI_3)
        .unwrap()
        .with_transform(Transform::translation(0.0, -1.0, -5.0));

    let image = camera.render(&world, raytracer::camera::RenderProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
