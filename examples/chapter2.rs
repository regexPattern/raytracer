use raytracer::{
    canvas::Canvas,
    color,
    tuple::{Point, Tuple, Vector},
};

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
        velocity: Vector::new(1.0, 1.8, 0.0).normalize().unwrap() * 11.25,
    };

    let e = Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let mut c = Canvas::new(900, 550);

    while p.is_in_the_air() {
        let Point(Tuple { x, y, .. }) = p.position;

        c.write_pixel(x as u32, c.height - y as u32, color::consts::WHITE);

        p.tick(&e);
    }

    let img = c.to_image();
    img.save("image.ppm").unwrap();
}
