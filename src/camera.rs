#![allow(clippy::cast_possible_truncation)]

use std::sync::{Arc, Mutex};

use crate::canvas::Canvas;
use crate::matrix::{self, Matrix};
use crate::ray::Ray;
use crate::tuple::Point;
use crate::world::{self, World};

#[derive(Debug)]
pub struct Camera {
    pub transform: Matrix<4, 4>,
    hsize: u32,
    vsize: u32,
    pixel_size: f64,
    half_height: f64,
    half_width: f64,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Self {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let pixel_size = half_width * 2.0 / f64::from(hsize);
        let transform = matrix::IDENTITY4X4;

        Self {
            transform,
            hsize,
            vsize,
            pixel_size,
            half_height,
            half_width,
        }
    }

    fn ray_for_pixel(&self, px: u32, py: u32) -> Ray {
        let xoffset = (f64::from(px) + 0.5) * self.pixel_size;
        let yoffset = (f64::from(py) + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);

        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let mutex = Arc::new(Mutex::new(&mut image));

        std::thread::scope(|scope| {
            let mut handles = Vec::new();

            for y in 0..self.vsize {
                let mutex = Arc::clone(&mutex);
                let handle = scope.spawn(move || {
                    let mut pixels = Vec::with_capacity(self.hsize as usize);

                    for x in 0..self.hsize {
                        let ray = self.ray_for_pixel(x, y);
                        let color = world.color_at(&ray, world::REFLECTION_LIMIT);
                        pixels.push(color);
                    }

                    let mut image = mutex.lock().unwrap();
                    for (x, color) in pixels.into_iter().enumerate() {
                        image.write_pixel(x as u32, y, color);
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.join().unwrap();
            }
        });

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::light::Light;
    use crate::material::{Material, Texture};
    use crate::shape::{Shape, Shapes, Sphere};
    use crate::tuple::Vector;
    use crate::{assert_approx, color};

    use super::*;

    fn test_default_world() -> World {
        let inner_sphere = Shapes::Sphere(Sphere(Shape {
            material: Material {
                diffuse: 0.7,
                specular: 0.2,
                texture: Texture::from(Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                }),
                ..Default::default()
            },
            ..Default::default()
        }));

        let outer_sphere = Shapes::Sphere(Sphere(Shape {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        }));

        let main_light = Light {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        World {
            objects: vec![inner_sphere, outer_sphere],
            lights: vec![main_light],
        }
    }

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::FRAC_PI_2;

        let camera = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, 160);
        assert_eq!(camera.vsize, 120);
        assert_eq!(camera.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let camera = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);

        assert_approx!(camera.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let camera = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);

        assert_approx!(camera.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);

        let ray = camera.ray_for_pixel(0, 0);

        assert_eq!(ray.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(ray.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut camera = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        camera.transform =
            Matrix::rotation_y(std::f64::consts::FRAC_PI_4) * Matrix::translation(0.0, -2.0, 5.0);

        let ray = camera.ray_for_pixel(100, 50);

        assert_eq!(ray.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            ray.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let world = test_default_world();

        let mut camera = Camera::new(11, 11, std::f64::consts::FRAC_PI_2);

        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        camera.transform = Matrix::view(from, to, up);

        let image = camera.render(&world);

        assert_eq!(
            *image.pixel_at(5, 5),
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        );
    }
}
