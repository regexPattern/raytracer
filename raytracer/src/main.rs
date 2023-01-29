use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    shape::{Cylinder, Group, Shape, Sphere},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    let sphere = Shape::Sphere(Sphere::new(
        Default::default(),
        Transform::scaling(2.0, 1.0, 1.25).unwrap(),
    ));
    let cylinder = Shape::Cylinder(Cylinder::new(
        Default::default(),
        Default::default(),
        -1.5,
        1.5,
        false,
    ));

    let mut inner = Group::default().with_transform(
        Transform::scaling(2.0, 1.0, 1.0).unwrap()
            * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
    );
    inner.push(cylinder);

    let mut group = Group::default();
    group.push(sphere);
    group.push(Shape::Group(inner));

    group.change_transform(Transform::rotation_z(std::f64::consts::FRAC_PI_6));

    let light = PointLight {
        position: Point::new(5.0, 5.0, 5.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![Shape::Group(group)],
        lights: vec![light],
    };

    let camera = Camera::new(
        500,
        500,
        std::f64::consts::FRAC_PI_3,
        Transform::translation(0.0, 0.0, -10.0),
    )
    .unwrap();

    let image = camera
        .render(&world, raytracer::scene::SceneProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
