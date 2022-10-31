use raytracer::camera::Camera;
use raytracer::color::{self, Color, RGB};
use raytracer::light::PointLight;
use raytracer::material::{Material, Texture};
use raytracer::matrix::Matrix;
use raytracer::pattern::{Checker, Design, Gradient, Pattern};
use raytracer::shape::{Figure, Plane, Shape, Sphere};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;

fn main() {
    let objects = vec![
        Shape::Sphere(Sphere(Figure {
            material: Material {
                texture: Texture::Pattern(Pattern::Gradient(Gradient(Design {
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
                }))),
                ..Default::default()
            },
            transform: Matrix::translation(0.0, 1.0, 0.0),
        })),
        Shape::Plane(Plane(Figure {
            material: Material {
                texture: Texture::Pattern(Pattern::Checker(Checker(Design::new(
                    Color::from(RGB {
                        red: 150,
                        green: 50,
                        blue: 50,
                    }),
                    Color::from(RGB {
                        red: 150,
                        green: 150,
                        blue: 150,
                    }),
                )))),
                ..Default::default()
            },
            ..Default::default()
        })),
    ];

    let lights = vec![PointLight {
        position: Point::new(7.0, 10.0, 7.0),
        intensity: color::WHITE,
    }];

    let world = World { objects, lights };

    // let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    let mut camera = Camera::new(3840, 2160, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        Point::new(1.0, 5.0, 5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    println!("{}", image.to_ppm());
}
