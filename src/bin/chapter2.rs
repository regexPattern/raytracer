use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::tuple::{Point, Vector};
use std::fs::File;

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
        position: Point::new(0.0, 20.0, 0.0),
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);
    let color = Color::new(1.0, 1.0, 1.0);

    // TODO: Deberia considerar podes escribir directamente `Points` al canvas?
    // NOTE: Si hago esto no podria restarle las dimensiones del canvas a la posicion relativa del
    // projectile.
    while canvas
        .insert(p.position.tuple.x as i32, p.position.tuple.y as i32, color)
        .is_ok()
    {
        println!("{:?}", p.position);
        tick(&mut p, &env);
        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    let mut f = File::create("/home/carlosecp/Pictures/canvas.ppm").unwrap();
    canvas.to_ppm(&mut f).unwrap();
}
