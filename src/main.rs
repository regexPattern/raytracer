use raytracer::{
    camera::{Camera, RenderProgress},
    color::{self, Color},
    light::PointLight,
    material::Material,
    shape::{Object, Plane, Shape, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let floor = Shape::Plane(Plane(Object {
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.9,
                blue: 0.9,
            },
            specular: 0.0,
            ..Default::default()
        },
        ..Default::default()
    }));

    let left_wall = Shape::Plane(Plane(Object {
        transform: Transform::translation(0.0, 0.0, 5.0)
            * Transform::rotation_y(-std::f64::consts::FRAC_PI_4)
            * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
        ..floor.as_ref().clone()
    }));

    let right_wall = Shape::Plane(Plane(Object {
        transform: Transform::translation(0.0, 0.0, 5.0)
            * Transform::rotation_y(std::f64::consts::FRAC_PI_4)
            * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
        ..floor.as_ref().clone()
    }));

    let middle = Shape::Sphere(Sphere(Object {
        transform: Transform::translation(-0.5, 1.0, 0.5),
        material: Material {
            color: Color {
                red: 0.1,
                green: 1.0,
                blue: 0.5,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    }));

    let right = Shape::Sphere(Sphere(Object {
        transform: Transform::translation(1.5, 0.5, -0.5)
            * Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
        material: Material {
            color: Color {
                red: 0.5,
                green: 1.0,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    }));

    let left = Shape::Sphere(Sphere(Object {
        transform: Transform::translation(-1.5, 0.33, -0.75)
            * Transform::try_scaling(0.33, 0.33, 0.33).unwrap(),
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.8,
                blue: 0.1,
            },
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        },
    }));

    let light1 = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let light2 = PointLight {
        position: Point::new(10.0, 10.0, -10.0),
        intensity: color::consts::RED,
    };

    let objects = vec![floor, left_wall, right_wall, middle, right, left];
    let lights = vec![light1, light2];

    let world = World { objects, lights };

    let mut camera = Camera::try_new(1280, 720, std::f64::consts::FRAC_PI_3).unwrap();

    camera.transform = Transform::try_view(
        Point::new(0.15, 10.0, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(1.0, 0.0, 1.0),
    )
    .unwrap();

    let image = camera.render(&world, RenderProgress::Enable).to_image();

    image.save("image.png").unwrap();
}
