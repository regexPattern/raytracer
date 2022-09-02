use raytracer::canvas::{Canvas, Color};
use raytracer::tuple::Tuple;
use std::thread;
use std::time::Duration;
use tempfile::NamedTempFile;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(p: Projectile, env: &Environment) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::normalize(Tuple::vector(1.0, 1.8, 0.0)) * 11.25,
    };

    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while canvas
        .write_pixel(
            p.position.x as u32,
            canvas.height - p.position.y as u32,
            Color::white(),
        )
        .is_ok()
    {
        println!("{:?}", &p.position);
        p = tick(p, &env);

        thread::sleep(Duration::from_millis(10));
    }

    let mut file = NamedTempFile::new().unwrap();
    canvas.to_ppm(&mut file);

    let (_, path) = file.keep().unwrap();
    println!("Canvas written to: {:?}", path);
}
