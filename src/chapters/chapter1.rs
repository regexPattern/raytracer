use crate::tuple::{Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

impl Projectile {
    fn has_hit_the_ground(&self) -> bool {
        self.position.0.y <= 0.0
    }
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

pub fn run() {
    let mut proj = Projectile {
        position: Point::new(0.0, 1.0, 0.0),
        velocity: Vector::new(1.0, 1.0, 0.0).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    while !proj.has_hit_the_ground() {
        println!(
            "{{ x: {:.3}, y: {:.3} }}",
            proj.position.0.x, proj.position.0.y
        );
        tick(&env, &mut proj);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
