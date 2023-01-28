use std::num::NonZeroUsize;

use raytracer::{
    camera::Camera,
    color::{self, Color},
    light::PointLight,
    material::Material,
    pattern::{Pattern3D, Schema},
    scene::SceneProgress,
    shape::{Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let floor = Shape::Plane(Plane::new(
        Material {
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ..Default::default()
        },
        Default::default(),
    ));

    let striped_sphere = Shape::Sphere(Sphere::new(
        Material {
            pattern: Pattern3D::Stripe(Schema::new(
                color::consts::WHITE,
                Color {
                    red: 0.7,
                    green: 0.4,
                    blue: 0.1,
                },
                Transform::scaling(0.25, 0.25, 0.25).unwrap()
                    * Transform::rotation_z(-std::f64::consts::FRAC_PI_4)
                    * Transform::translation(0.5, 0.0, 0.0),
            )),
            specular: 0.3,
            ..Default::default()
        },
        Transform::translation(0.0, 1.0, 0.0),
    ));

    let right_light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::RED,
    };

    let left_light = PointLight {
        position: Point::new(-10.0, 10.0, 10.0),
        intensity: Color {
            red: 0.3216,
            green: 0.6784,
            blue: 0.03,
        },
    };

    let world = World {
        objects: vec![floor, striped_sphere],
        lights: vec![left_light, right_light],
    };

    let camera = Camera::new(
        NonZeroUsize::new(1280).unwrap(),
        NonZeroUsize::new(720).unwrap(),
        std::f64::consts::FRAC_PI_3,
        Transform::view(
            Point::new(0.0, 3.0, 5.0),
            Point::new(0.0, 1.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    )
    .unwrap();

    let image = camera.render(&world, SceneProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
