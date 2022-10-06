use std::fs::File;
use std::io::Write;

use crate::canvas::Canvas;
use crate::color;
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
        velocity: Vector::new(1.0, 1.8, 0.0).normalize() * 11.25,
    };

    let env = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut canvas = Canvas::new(900, 550);

    while !proj.has_hit_the_ground() {
        let pixel_x = (proj.position.0.x) as u32;
        let pixel_y = (canvas.height as f64 - proj.position.0.y) as u32;

        canvas.write_pixel(pixel_x, pixel_y, color::WHITE);

        tick(&env, &mut proj);
    }

    let ppm = canvas.to_ppm();
    let mut file = File::create("chapter2.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
}
