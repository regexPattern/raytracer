use std::error::Error;
use std::fs::File;

use raytracer::camera::Camera;
use raytracer::light::PointLight;
use raytracer::shape::Sphere;
use raytracer::transformation;
use raytracer::tuple::{Color, Point, Vector};
use raytracer::world::World;

fn main() -> Result<(), Box<dyn Error>> {
    let s1 = Sphere {
        transform: transformation::scaling(2.0, 1.0, 2.0),
        ..Sphere::default()
    };

    let objects = vec![s1];
    let light = PointLight::new(Point::new(0.0, 0.0, -5.0), Color::new(1.0, 0.0, 0.0));

    let world = World::new(objects, light);

    let mut camera = Camera::new(200, 200, std::f64::consts::FRAC_PI_3);
    camera.transform = transformation::view(
        Point::new(2.0, 0.0, -5.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    if let Ok(canvas) = camera.render(&world) {
        let mut file = File::create("image.ppm")?;
        canvas.to_ppm(&mut file)?;
    }

    Ok(())
}
