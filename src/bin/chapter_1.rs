use std::thread;
use raytracer::tuple::{Point, Vector};

#[derive(Copy, Clone, Debug)]
struct Projectile {
    position: Point,
    velocity: Vector,
}

#[derive(Copy, Clone)]
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn main() {
    let mut proj = Projectile {
        position: Point::new(0., 1., 0.),
        velocity: Vector::new(1., 1., 0.).normalize(),
    };

    let env = Environment {
        gravity: Vector::new(0., -0.1, 0.),
        wind: Vector::new(-0.01, 0., 0.),
    };

    let mut ticks = 0;

    loop {
        tick(env, &mut proj);
        ticks += 1;

        println!(
            "\rProjectile {{ x: {:.2}, y: {:.2} }}",
            proj.position.x, proj.position.y
        );

        thread::sleep(std::time::Duration::from_secs(1));

        if proj.position.y <= 0. {
            break;
        }
    }

    println!("Ticks: {}", ticks);
}
