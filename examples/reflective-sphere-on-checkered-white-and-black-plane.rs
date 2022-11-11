use raytracer::{
    camera::Camera,
    color,
    light::PointLight,
    material::{Material, Texture},
    matrix::Matrix,
    pattern::{Checker, Scheme},
    shape::{Figure, Plane, Shapes, Sphere},
    tuple::{Point, Vector},
    world::World,
};

fn main() {
    let sphere = Shapes::Sphere(Sphere(Figure {
        material: Material {
            reflective: 0.5,
            texture: Texture::from(color::WHITE),
            ..Default::default()
        },
        transform: Matrix::translation(0.0, 1.0, 0.0),
    }));

    let plane = Shapes::Plane(Plane(Figure {
        material: Material {
            texture: Texture::from(Checker(Scheme::new(color::WHITE, color::BLACK))),
            ..Default::default()
        },
        ..Default::default()
    }));

    let light = PointLight {
        position: Point::new(3.0, 3.0, 3.0),
        intensity: color::WHITE,
    };

    let objects = vec![sphere, plane];
    let lights = vec![light];

    let world = World { objects, lights };

    let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(0.0, 3.0, 5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);
    println!("{}", image.to_ppm());
}
