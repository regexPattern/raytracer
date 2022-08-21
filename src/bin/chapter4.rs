use raytracer::canvas::{Canvas, Color};
use raytracer::matrix::Transformation;
use raytracer::tuple::Tuple;
use std::fs::File;

fn main() {
    let mut canvas = Canvas::new(256, 256);

    let steps = 12;
    let full_circle = 2.0 * std::f64::consts::PI;

    let rotation = Transformation::rotation_z(-full_circle / steps as f64);
    let translation = Transformation::translation(127.0, 127.0, 0.0);
    let mut position = Tuple::point(0.0, 1.0, 0.0);

    for _ in 0..steps {
        position = translation * position;

        canvas.write_pixel(
            position.x as u32,
            position.y as u32,
            Color::new(1.0, 1.0, 1.0),
        );

        position = rotation * position;
    }

    let mut file = File::create("./image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}
