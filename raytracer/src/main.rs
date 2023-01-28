use std::num::NonZeroUsize;

use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    obj_model::OBJModel,
    shape::Shape,
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let file = std::fs::read_to_string("dragon.obj").unwrap();
    let model = OBJModel::new(&file, raytracer::scene::SceneProgress::Enable).unwrap();
    let mut group = model.build(Transform::translation(4.0, 4.0, 4.0));

    group.divide(150);

    let light = PointLight {
        position: Point::new(10.0, 10.0, 10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![Shape::Group(group)],
        lights: vec![light],
    };

    let camera = Camera::new(
        NonZeroUsize::new(500).unwrap(),
        NonZeroUsize::new(500).unwrap(),
        std::f64::consts::FRAC_PI_2,
        Transform::view(
            Point::new(10.0, 10.0, 10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    )
    .unwrap();

    let image = camera
        .render(&world, raytracer::scene::SceneProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
