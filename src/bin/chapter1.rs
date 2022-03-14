use raytracer::coordinate::{Point, Vector};
use std::fmt;

struct Projectile {
    position: Point,
    velocity: Vector,
}

impl fmt::Debug for Projectile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Projectile")
            .field(
                "position",
                &format_args!(
                    "{{ x: {:.2}, y: {:.2} }}",
                    self.position.get_coord_x(),
                    self.position.get_coord_y()
                ),
            )
            .field(
                "velocity",
                &format_args!(
                    "{{ x: {:.2}, y: {:.2} }}",
                    self.velocity.get_coord_x(),
                    self.velocity.get_coord_y()
                ),
            )
            .finish()
    }
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(proj: &mut Projectile, env: &Environment) {
    // TODO: Implemente the `AddAsign` trait.
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

    while proj.position.get_coord_y() >= 0. {
        ticks += 1;

        println!("{:?}", proj);

        tick(&mut proj, &env);
    }

    println!("Took {} ticks to hit the ground.", ticks);
}
