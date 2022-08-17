use std::fs::File;

use raytracer::canvas::Canvas;
use raytracer::tuple::{Color, Point, Tuple, Vector};

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
        position: Point::new(0.0, 20.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while projectile.position.tuple.y < (canvas.height - 20).into()
        && projectile.position.tuple.y > 10.0
    {
        tick(&mut projectile, &env);

        let Tuple { x, y, .. } = projectile.position.tuple;
        canvas.write_pixel(
            x as i32,
            canvas.height - y as i32,
            Color::new(1.0, 1.0, 1.0),
        );
    }

    let mut f = File::create("/home/carlosecp/Pictures/canvas.ppm").unwrap();
    canvas.to_ppm(&mut f).unwrap();
}
