use raytracer::{
    camera::{Camera, CameraBuilder},
    color,
    light::PointLight,
    obj_model::{OBJModel, OBJModelBuilder},
    shape::{group::Group, Shape},
    transform::Transform,
    tuple::Point,
    world::World,
};

fn main() {
    let file = std::fs::read_to_string("al.obj").unwrap();
    let model = OBJModel::try_from(OBJModelBuilder {
        content: &file,
        transform: Transform::rotation_y(std::f64::consts::FRAC_PI_2)
            * Transform::scaling(2.0, 2.0, 2.0).unwrap(),
    })
    .unwrap();

    let mut model = Group::from(model);
    model.divide(300);

    let light = PointLight {
        position: Point::new(5.0, 5.0, 5.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![Shape::Group(model)],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        image_width: 200,
        image_height: 200,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::translation(0.0, 0.0, -20.0),
    })
    .unwrap();

    let image = camera
        .render(&world, raytracer::scene::SceneProgress::Enable)
        .to_image();
    image.save("image.png").unwrap();
}
