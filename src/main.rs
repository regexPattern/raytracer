use raytracer::camera::Camera;
use raytracer::light::Light;
use raytracer::material::{Material, Texture};
use raytracer::matrix::Matrix;
use raytracer::pattern::{Checker, Scheme};
use raytracer::shape::{Plane, Shape, Shapes, Sphere};
use raytracer::tuple::{Point, Vector};
use raytracer::world::World;
use raytracer::{color, matrix};

fn main() {
    let main_light = Light {
        position: Point::new(7.0, 10.0, 7.0),
        intensity: color::WHITE,
    };

    let world = World {
        objects: build_objects(),
        lights: vec![main_light],
    };

    let mut camera = Camera::new(1280, 720, std::f64::consts::FRAC_PI_3);
    // let mut camera = Camera::new(3840, 2160, std::f64::consts::FRAC_PI_3);
    camera.transform = Matrix::view(
        // TODO: Deberia hacer que estos valores fueran enteros para ser honesto.
        Point::new(0.0, 3.0, 1.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    let image = camera.render(&world);

    println!("{}", image.to_ppm());
}

fn build_objects() -> Vec<Shapes> {
    let floor = Shapes::from(Plane(Shape {
        material: Material {
            texture: Texture::from(Checker(Scheme {
                a: color::WHITE,
                b: color::BLACK,
                transform: matrix::IDENTITY4X4,
            })),
            ..Default::default()
        },
        ..Default::default()
    }));

    let reflective_sphere = build_reflective_sphere();

    vec![floor, reflective_sphere]
}

fn build_reflective_sphere() -> Shapes {
    // Supongo que en el UI puedo hacer una funcionalidad para ajustar los paremetros de cada
    // propiedad. Asi por ejemplo, hago un constructor del material, otro para la trasformacion,
    // etc.
    let reflective = 0.5;
    // Realmente no terminaria usando ..Default::default(), sino que cada parametro seria pasado
    // con el valor que le de el usuario desde el UI. El defualt() podria ser usado para
    // inicializar los valores en el UI por ejemplo.
    let material = Material {
        reflective,
        ..Default::default()
    };

    let (x, y, z) = (0.0, 1.0, 0.0);
    let positioning = Matrix::translation(x, y, z);
    // El tema es que que transformacion aplicaria primero? Tengo que dejar que el usuario decida
    // esto.
    // let rotation = Matrix::rotation_x(0.0);
    // let transform = rotation * positioning;

    let scaling = Matrix::scaling(1.0, 1.0, 1.0);
    // Probablemente positioning y scaling deberian ir en este orden.
    let transform = scaling * positioning;

    Shapes::from(Sphere(Shape {
        material,
        transform,
    }))
}
