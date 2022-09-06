use std::thread;
use std::time::Duration;

use raytracer::tuple::{Point, Vector};

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

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while p.position.0.y >= 0.0 {
        println!("{:?}", &p.position);
        p = tick(p, &env);

        thread::sleep(Duration::from_millis(50));
    }
}
