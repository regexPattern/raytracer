use raytracer::tuple::Tuple;
use std::thread;
use std::time::Duration;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(p: Projectile, env: &Environment) -> Projectile {
    let position = p.position + &p.velocity;
    let velocity = p.velocity + &env.gravity + &env.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut p = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::normalize(Tuple::vector(1.0, 1.0, 0.0)),
    };

    let env = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };

    while p.position.y >= 0.0 {
        println!("{:?}", &p.position);
        p = tick(p, &env);

        thread::sleep(Duration::from_millis(50));
    }
}
