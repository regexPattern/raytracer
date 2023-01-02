use raytracer::{
    canvas::Canvas,
    color::{self, Color},
    intersections::Intersection,
    light::PointLight,
    material::Material,
    ray::Ray,
    sphere::Sphere,
    tuple::Point,
};

fn main() {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixels = 500;
    let pixel_size = wall_size / f64::from(canvas_pixels);
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);

    let sphere = Sphere {
        material: Material {
            color: Color {
                red: 1.0,
                green: 0.2,
                blue: 1.0,
            },
            ..Default::default()
        },
        ..Default::default()
    };

    let light = PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::consts::WHITE,
    };

    for y in 0..canvas_pixels {
        let world_y = half - pixel_size * f64::from(y);

        for x in 0..canvas_pixels {
            let world_x = -half + pixel_size * f64::from(x);
            let position = Point::new(world_x, world_y, wall_z);

            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).normalize().unwrap(),
            };

            let xs = sphere.intersect(&ray);

            if let Some(hit) = Intersection::hit(xs) {
                let point = ray.position(hit.t);
                let normal = hit.object.normal_at(point);
                let eye = -ray.direction;

                let color = hit
                    .object
                    .material
                    .lighting(&light, point, eye, normal)
                    .unwrap();

                canvas.write_pixel(x, y, color);
            }
        }
    }

    let image = canvas.to_image();
    image.save("image.png").unwrap();
}
