use std::fs::File;

use raytracer::canvas::{Canvas, Color};
use raytracer::matrix::Transformation;
use raytracer::tuple::Tuple;

fn main() {
    let mut canvas = Canvas::new(256, 256);
    let center = 127.0;

    let mut point = Tuple::point(0.0, 0.0, 1.0);
    let rotate_y = Transformation::rotation_y(std::f64::consts::FRAC_PI_6);

    let radius = 3.0 / 8.0 * canvas.width as f64;

    for _ in 0..12 {
        let Tuple { x, z, .. } = point * radius;

        canvas
            .write_pixel(
                (x + center) as u32,
                (z + center) as u32,
                Color::new(1.0, 1.0, 1.0),
            )
            .unwrap();

        point = rotate_y * point;
    }

    let mut file = File::create("image.ppm").unwrap();
    canvas.to_ppm(&mut file);
}
