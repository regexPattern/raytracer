use raytracer::{
    camera::{Camera, RenderProgress},
    color::{self, Color},
    light::PointLight,
    material::Material,
    pattern::{Pattern, Schema},
    shape::{Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let floor = Shape::Plane(Plane::default());

    let left_wall = Shape::Plane(Plane::new(
        Material {
            pattern: Pattern::Checker(Schema::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            )),
            ..Default::default()
        },
        Transform::translation(0.0, 1.0, 0.0) * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
    ));

    let right_wall = Shape::Plane(Plane::new(
        left_wall.as_ref().material.clone(),
        Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    ));

    let metallic_sphere = Shape::Sphere(Sphere::new(
        Material {
            reflectivity: 0.25,
            pattern: Pattern::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            }),
            ..Default::default()
        },
        Transform::translation(4.0, 1.0, -4.0),
    ));

    let red_sphere = Shape::Sphere(Sphere::new(
        Material {
            pattern: Pattern::Solid(color::consts::RED),
            ..Default::default()
        },
        Transform::translation(4.0, 0.5, -6.0) * Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
    ));

    let blue_sphere = Shape::Sphere(Sphere::new(
        Material {
            pattern: Pattern::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.9,
            }),
            ..Default::default()
        },
        Transform::translation(6.0, 0.25, -4.5) * Transform::try_scaling(0.25, 0.25, 0.25).unwrap(),
    ));

    let light = PointLight {
        position: Point::new(5.0, 5.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![
            floor,
            left_wall,
            right_wall,
            metallic_sphere,
            red_sphere,
            blue_sphere,
        ],
        lights: vec![light],
    };

    let camera = Camera::try_new(
        1280,
        720,
        std::f64::consts::FRAC_PI_3,
        Transform::try_view(
            Point::new(10.0, 3.0, -10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    )
    .unwrap();

    let image = camera.render(&world, RenderProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
