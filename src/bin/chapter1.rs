use raytracer::tuple::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(p: &mut Projectile, env: &Environment) {
    p.position = p.position + p.velocity;
    p.velocity = p.velocity + env.gravity + env.wind;
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

    while p.position.tuple.y >= 0.0 {
        println!("{:?}", p.position);
        tick(&mut p, &env);
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
