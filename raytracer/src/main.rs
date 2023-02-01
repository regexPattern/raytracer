use raytracer::{
    camera::{Camera, CameraBuilder},
    color,
    light::{Light, PointLight},
    material::Material,
    pattern::Pattern3DSpec,
    shape::{Cube, Plane, Shape, ShapeBuilder},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let light = Light::Point(PointLight {
        position: Point::new(5.0, 5.0, 5.0),
        intensity: color::consts::WHITE,
    });

    let cube = Shape::Cube(Cube::from(ShapeBuilder {
        material: Material {
            pattern: raytracer::pattern::Pattern3D::Solid(color::consts::RED),
            transparency: 0.0,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 1.0, 0.0),
    }));

    let plane = Shape::Plane(Plane::from(ShapeBuilder {
        material: Material {
            pattern: raytracer::pattern::Pattern3D::Checker(Pattern3DSpec::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            )),
            ..Default::default()
        },
        ..Default::default()
    }));

    let world = World {
        objects: vec![cube, plane],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        width: 200,
        height: 200,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(5.0, 5.0, 5.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    let image = camera.render(&world).to_image();
    image.save("image.png").unwrap();
}
