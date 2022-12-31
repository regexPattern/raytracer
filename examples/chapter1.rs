use raytracer::tuple::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    fn tick(&mut self, env: &Environment) {
        self.position = self.position + self.velocity;
        self.velocity = self.velocity + env.gravity + env.wind;
    }

    fn is_in_the_air(&self) -> bool {
        self.position.0.y > 0.0
    }
}

fn main() {
    let mut p = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0),
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while p.is_in_the_air() {
        println!("{:?}", p.position);
        p.tick(&e);
    }
}
