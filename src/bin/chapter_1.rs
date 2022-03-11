use raytracer::{Point, Vector};

// Putting It Together
//
// Try playing with this little program, firing virtual projectiles and seeing
// how fat they go. It'll let you exercise the vector and point routines you've
// written. Start with the following two data structures:
//
//  * A projectile has a position (a point) and a velocity (a vector).
//  * An environment has gravity (a vector) and wind (a vector).
//
// Then, add a `tick(environment, projectile)` function which returns a new
// projectile, representing the given projectile after one unit of time has
// passed. (The actual units here don't really matter - maybe they're seconds,
// or milliseconds. Whatever. We'll just call them "ticks".)

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: Environment, proj: Projectile) -> Projectile {
}

fn main() {
    // tick();
}
