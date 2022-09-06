use std::error::Error;
use std::fs::File;

use raytracer::canvas::Canvas;
use raytracer::transformation;
use raytracer::tuple::{Color, Point, Tuple};

fn main() -> Result<(), Box<dyn Error>> {
    let mut canvas = Canvas::new(256, 256);
    let center = 127.0;

    let mut point = Point::new(0.0, 1.0, 0.0);
    let rotate_z = transformation::rotation_z(std::f64::consts::FRAC_PI_6);

    let radius = 3.0 / 8.0 * canvas.width as f64;

    for _ in 0..12 {
        let Point(Tuple { x, y, .. }) = point * radius;

        canvas
            .write_pixel((x + center) as u32, (y + center) as u32, Color::white())
            .unwrap();

        point = rotate_z * point;
    }

    let mut file = File::create("image.ppm")?;
    canvas.to_ppm(&mut file)?;
    Ok(())
}
