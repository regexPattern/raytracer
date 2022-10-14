use std::fs::File;
use std::io::prelude::*;

use raytracer::canvas::Canvas;
use raytracer::color;
use raytracer::matrix::Matrix;
use raytracer::tuple::Vector;

fn main() {
    let mut v = Vector::new(0.0, 1.0, 0.0);
    let ticks = 100;

    let mut canvas = Canvas::new(500, 500);
    let radius = 3.0 * f64::from(canvas.width) / 8.0;

    let transform = Matrix::rotation_z(-2.0 * std::f64::consts::PI / f64::from(ticks));

    for _ in 0..ticks {
        let x = v.0.x * radius + f64::from(canvas.width / 2);
        let y = v.0.y * radius + f64::from(canvas.height / 2);

        canvas.write_pixel(x as u32, y as u32, color::WHITE);

        v = transform * v;
    }

    let mut file = File::create("image.ppm").unwrap();
    let _ = file.write(canvas.to_ppm().as_bytes()).unwrap();
}
