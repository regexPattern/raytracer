#![allow(unused, dead_code)]

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    shape::{BaseShape, Cylinder, Group, Shape},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn hexagon_corner() -> Shape {
    Shape::Sphere(BaseShape {
        transform: Transform::translation(0.0, 0.0, -1.0)
            * Transform::try_scaling(0.25, 0.25, 0.25).unwrap(),
        ..Default::default()
    })
}

fn hexagon_edge() -> Shape {
    Shape::Cylinder(Cylinder {
        base_shape: BaseShape {
            transform: Transform::translation(0.0, 0.0, -1.0)
                * Transform::rotation_y(-std::f64::consts::FRAC_PI_6)
                * Transform::rotation_z(-std::f64::consts::FRAC_PI_2)
                * Transform::try_scaling(0.25, 1.0, 0.25).unwrap(),
            ..Default::default()
        },
        minimum: 0.0,
        maximum: 1.0,
        closed: false,
    })
}

fn hexagon_side() -> Shape {
    Shape::Group(Group::new(
        [hexagon_corner(), hexagon_edge()],
        Default::default()
    ))
}

fn hexagon() -> Shape {
    let mut hex = Group::default();

    for n in 0..6 {
        let mut side = hexagon_side();
        side.set_transform(Transform::rotation_y(f64::from(n) * std::f64::consts::FRAC_PI_3));
        hex.add_child(side);
    }

    Shape::Group(hex)
}

fn main() {
    let light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![hexagon()],
        lights: vec![light],
    };

    let mut shape = Shape::Sphere(Default::default());
    shape.set_transform(Transform::translation(10.0, 15.0, 20.0));

    let camera = Camera::try_new(500, 500, std::f64::consts::FRAC_PI_3)
        .unwrap()
        .with_transform(
            Transform::try_view(
                Point::new(5.0, 5.0, 0.0),
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
