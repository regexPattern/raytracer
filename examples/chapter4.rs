use raytracer::{canvas::Canvas, color, transform::Transform, tuple::Vector};

const TICKS: u32 = 12;

fn main() {
    let mut c = Canvas::new(100, 100);
    let mut v = Vector::new(0.0, 1.0, 0.0);
    let rotation_z = Transform::rotation_z(2.0 * std::f64::consts::PI / f64::from(TICKS));

    let (width, height) = (f64::from(c.width), f64::from(c.height));

    let half = if width > height {
        height / 2.0 - height * 0.1
    } else {
        width / 2.0 - width * 0.1
    };

    for _ in 0..TICKS {
        c.write_pixel(
            ((v.0.x * half) + width / 2.0) as u32,
            ((v.0.y * half) + height / 2.0) as u32,
            color::consts::WHITE,
        );

        v = rotation_z * v;
    }

    let image = c.to_image();
    image.save("image.png").unwrap();
}
