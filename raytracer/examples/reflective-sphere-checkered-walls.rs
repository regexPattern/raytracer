use raytracer::{
    camera::{Camera, RenderProgress},
    color::{self, Color},
    light::PointLight,
    material::Material,
    pattern::{Pattern3D, Pattern3D},
    shape::{Object, Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

// const RESOLUTION: (u32, u32) = (3840, 2160);
// const RESOLUTION: (u32, u32) = (1280, 720);
const RESOLUTION: (u32, u32) = (1500, 500);
// const RESOLUTION: (u32, u32) = (400, 225);

fn main() {
    let reflective_sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            reflectivity: 0.3,
            transparency: 1.0,
            index_of_refraction: 1.06,
            specular: 0.01,
            pattern: Pattern3D::Solid(Color {
                red: 0.1,
                green: 0.1,
                blue: 0.2,
            }),
            ..Default::default()
        },
        transform: Transform::translation(6.0, 1.0, -6.0),
    }));

    let red_sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::RED),
            ..Default::default()
        },
        transform: Transform::translation(4.0, 0.5, -6.0)
            * Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
    }));

    let blue_sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.9,
            }),
            ..Default::default()
        },
        transform: Transform::try_scaling(0.75, 0.75, 0.75).unwrap()
            * Transform::translation(7.0, 1.0, -4.5),
    }));

    let green_sphere = Shape::Sphere(Sphere(Object {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.5373,
                green: 0.6745,
                blue: 0.4627,
            }),
            ..Default::default()
        },
        transform: Transform::translation(3.0, 1.0, -3.0),
    }));

    let floor = Shape::Plane(Plane(Object {
        material: Material {
            pattern: Pattern3D::Checker(Pattern3D::new(color::consts::WHITE, color::consts::BLACK)),
            ..Default::default()
        },
        ..Default::default()
    }));

    let left_wall = Shape::Plane(Plane(Object {
        transform: Transform::translation(0.0, 1.0, 0.0)
            * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    }));

    let right_wall = Shape::Plane(Plane(Object {
        transform: Transform::rotation_x(std::f64::consts::FRAC_PI_2),
        ..left_wall.as_ref().clone()
    }));

    let left_light = PointLight {
        position: Point::new(5.0, 5.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let objects = vec![
        reflective_sphere,
        red_sphere,
        blue_sphere,
        green_sphere,
        floor,
        left_wall,
        right_wall,
    ];
    let lights = vec![left_light];

    let world = World { objects, lights };

    let mut camera =
        Camera::try_new(RESOLUTION.0, RESOLUTION.1, std::f64::consts::FRAC_PI_3).unwrap();
    camera.transform = Transform::rotation_x(std::f64::consts::FRAC_PI_2)
        * Transform::translation(-4.5, -12.0, 4.5);

    let image = camera.render(&world, RenderProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
