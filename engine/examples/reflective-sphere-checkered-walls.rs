use engine::{
    camera::Camera,
    color::{self, Color},
    light::PointLight,
    material::{Material, Texture},
    matrix::Matrix,
    pattern::{Checker, Pattern, Scheme},
    shape::{Figure, Plane, Shape, Sphere},
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let reflective_sphere = Shape::Sphere(Sphere(Figure {
        material: Material {
            reflective: 0.25,
            texture: Texture::Color(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            }),
            ..Default::default()
        },
        transform: Matrix::translation(4.0, 1.0, -4.0),
    }));

    let red_sphere = Shape::Sphere(Sphere(Figure {
        material: Material {
            texture: Texture::Color(color::RED),
            ..Default::default()
        },
        transform: Matrix::translation(4.0, 0.5, -6.0) * Matrix::scaling(0.5, 0.5, 0.5),
    }));

    let blue_sphere = Shape::Sphere(Sphere(Figure {
        material: Material {
            texture: Texture::Color(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.9,
            }),
            ..Default::default()
        },
        transform: Matrix::translation(6.0, 0.25, -4.5) * Matrix::scaling(0.25, 0.25, 0.25),
    }));

    let floor = Shape::Plane(Plane::default());

    let left_wall = Shape::Plane(Plane(Figure {
        material: Material {
            texture: Texture::Pattern(Pattern::Checker(Checker(Scheme::new(
                color::WHITE,
                color::BLACK,
            )))),
            ..Default::default()
        },
        transform: Matrix::translation(0.0, 1.0, 0.0)
            * Matrix::rotation_z(std::f64::consts::FRAC_PI_2),
    }));

    let right_wall = Shape::Plane(Plane(Figure {
        transform: Matrix::rotation_x(std::f64::consts::FRAC_PI_2),
        ..left_wall.figure()
    }));

    let main_light = PointLight {
        position: Point::new(5.0, 5.0, -10.0),
        intensity: color::WHITE,
    };

    let objects = vec![
        reflective_sphere,
        red_sphere,
        blue_sphere,
        floor,
        left_wall,
        right_wall,
    ];
    let lights = vec![main_light];

    let world = World { objects, lights };

    let mut camera = Camera::new(1920, 1080, -std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(10.0, 3.0, -10.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    )
    .unwrap();

    let image = camera.render(&world);
    println!("{}", image.to_ppm());
}
