use raytracer::{
    camera::{Camera, CameraBuilder},
    color,
    light::{Light, PointLight},
    model::{Model, OBJModelBuilder},
    shape::{Group, Shape},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    // Load the contents of the file.
    let model_spec = std::fs::read_to_string("daft_punk.oej").unwrap();

    // Parse the file and create a model. Also apply a transformation to it.
    let model = Model::try_from(OBJModelBuilder {
        model_spec: &model_spec,
        transform: Transform::translation(0.0, 0.5, 0.0),
    })
    .unwrap();

    // Create a group and optimize it.
    let mut group = Group::from(model);
    group.divide(64);

    let light = Light::Point(PointLight {
        position: Point::new(0.0, 7.0, 12.0),
        intensity: color::consts::WHITE,
    });

    // Convert the group to a `Shape` and add it to the world.
    let world = World {
        objects: vec![Shape::Group(group)],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        width: 1280,
        height: 720,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::translation(0.0, 0.0, -12.0),
    })
    .unwrap();

    let image = camera.render(&world).to_image();
    image.save("image.png").unwrap();
}
