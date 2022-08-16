use raytracer::tuple::{Point, Scalar, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(projectile: &mut Projectile, env: &Environment) {
    projectile.position = projectile.position + projectile.velocity;
    projectile.velocity = projectile.velocity + env.gravity + env.wind;
}

fn main() {
    let mut projectile = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while projectile.position.tuple.y >= 0.0 {
        tick(&mut projectile, &env);
        std::thread::sleep(std::time::Duration::from_millis(500));
        println!("{:?}", projectile.position);
    }
}
