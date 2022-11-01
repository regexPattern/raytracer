use raytracer::camera::Camera;
use raytracer::color::{self, Color, RGB};
use raytracer::light::Light;
use raytracer::material::{Material, Texture};
use raytracer::matrix::Matrix;
use raytracer::pattern::{Gradient, Scheme};
use raytracer::shape::{Shape, Shapes, Sphere};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

fn red_sphere() -> Shapes {
    let material = Material {
        texture: Texture::from(Gradient(Scheme {
            a: Color::from(RGB {
                red: 100,
                green: 50,
                blue: 50,
            }),
            b: Color::from(RGB {
                red: 200,
                green: 200,
                blue: 200,
            }),
            transform: Matrix::translation(1.0, 0.0, 0.0) * Matrix::scaling(2.0, 2.0, 2.0),
        })),
        ..Default::default()
    };

    let transform = Matrix::translation(0.0, 1.0, 0.0);

    Shapes::from(Sphere(Shape {
        material,
        transform,
    }))
}

fn main() {
    let objects = vec![red_sphere()];

    let lights = vec![Light {
        position: Point::new(7.0, 10.0, 7.0),
        intensity: color::WHITE,
    }];

    let world = World { objects, lights };

    let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    // let mut camera = Camera::new(3840, 2160, std::f64::consts::FRAC_PI_3);
    // TODO: Deberia pasar la transform en cuanto creo la camara? Creo que no.
    camera.transform = Matrix::view(
        Point::new(0.0, 5.0, 0.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    println!("{}", image.to_ppm());
}
