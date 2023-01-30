use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    obj_model::OBJModel,
    shape::{Cylinder, Group, Shape, Sphere},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    let file = std::fs::read_to_string("al.obj").unwrap();
    let mut model = OBJModel::new(&file, raytracer::scene::SceneProgress::Enable)
        .unwrap()
        .build(Transform::rotation_y(std::f64::consts::FRAC_PI_2));

    model.divide(64);

    let sphere = Shape::Sphere(
        Sphere::default().with_transform(Transform::scaling(2.0, 1.0, 1.25).unwrap()),
    );
    let cylinder = Shape::Cylinder(Cylinder::new(
        Default::default(),
        Default::default(),
        -1.5,
        1.5,
        false,
    ));

    let mut inner = Group::new(
        Transform::scaling(2.0, 1.0, 1.0).unwrap()
            * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
    );
    inner.push(cylinder);

    let mut group = Group::default();
    group.push(sphere);
    group.push(Shape::Group(inner));

    // group.replace_transform(Transform::rotation_z(std::f64::consts::FRAC_PI_6));

    let light = PointLight {
        position: Point::new(5.0, 5.0, 5.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![Shape::Group(model)],
        lights: vec![light],
    };

    let camera = Camera::new(
        200,
        200,
        std::f64::consts::FRAC_PI_3,
        Transform::translation(0.0, 0.0, -5.0),
    )
    .unwrap();

    let image = camera
        .render(&world, raytracer::scene::SceneProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
