use std::error::Error;
use std::fs::File;
use std::thread;
use std::time::Duration;

use raytracer::canvas::Canvas;
use raytracer::tuple::{Color, Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(p: Projectile, env: &Environment) -> Projectile {
    let position = p.position + p.velocity;
    let velocity = p.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while canvas
        .write_pixel(
            p.position.0.x as u32,
            canvas.height - p.position.0.y as u32,
            Color::white(),
        )
        .is_ok()
    {
        println!("{:?}", &p.position);
        p = tick(p, &env);

        thread::sleep(Duration::from_millis(10));
    }

    let mut file = File::create("image.ppm")?;
    canvas.to_ppm(&mut file)?;
    Ok(())
}
