use crate::canvas::Canvas;
use crate::matrix::{self, Matrix};
use crate::ray::Ray;
use crate::tuple::Point;
use crate::world::World;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    field_of_view: f64,
    pub transform: Matrix<4, 4>,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
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
            hsize,
            vsize,
            field_of_view,
            transform,
            pixel_size,
            half_width,
            half_height,
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

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;
    use crate::color::Color;
    use crate::tuple::Vector;

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::FRAC_PI_2;

        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert_eq!(c.field_of_view, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.transform, matrix::IDENTITY4X4);
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, std::f64::consts::FRAC_PI_2);

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, std::f64::consts::FRAC_PI_2);

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, std::f64::consts::FRAC_PI_2);
        c.transform =
            Matrix::rotation_y(std::f64::consts::FRAC_PI_4) * Matrix::translation(0.0, -2.0, 5.0);

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let world = World::default();
        let mut c = Camera::new(11, 11, std::f64::consts::FRAC_PI_2);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = Matrix::view(from, to, up);

        let image = c.render(&world);

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
