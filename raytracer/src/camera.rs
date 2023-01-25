use std::sync::{Arc, Mutex};

use indicatif::ProgressBar;
use rayon::ThreadPoolBuilder;

use crate::{canvas::Canvas, float, ray::Ray, transform::Transform, tuple::Point, world::World};

const DEFAULT_THREADS: usize = 8;

#[derive(Debug, PartialEq)]
pub enum CameraError {
    NullDimension,
    MultipleOfPiFieldOfView,
}

#[derive(Debug)]
pub struct Camera {
    hsize: u32,
    vsize: u32,
    field_of_view: f64,
    pixel_size: f64,
    half_height: f64,
    half_width: f64,
    transform: Transform,
    transform_inverse: Transform,
}

#[derive(Debug)]
pub enum RenderProgress {
    Enable,
    Disable,
}

impl PartialEq for Camera {
    fn eq(&self, other: &Self) -> bool {
        self.hsize == other.hsize
            && self.vsize == other.vsize
            && float::approx(self.field_of_view, other.field_of_view)
            && float::approx(self.pixel_size, other.pixel_size)
            && float::approx(self.half_width, other.half_width)
            && float::approx(self.half_height, other.half_height)
            && self.transform == other.transform
            && self.transform_inverse == other.transform_inverse
    }
}

impl Camera {
    pub fn try_new(
        hsize: u32,
        vsize: u32,
        field_of_view: f64,
        transform: Transform,
    ) -> Result<Self, CameraError> {
        if hsize * vsize == 0 {
            return Err(CameraError::NullDimension);
        }

        if float::approx(field_of_view % std::f64::consts::PI, 0.0) {
            return Err(CameraError::MultipleOfPiFieldOfView);
        }

        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);

        let (half_width, half_height) = if aspect < 1.0 {
            (half_view * aspect, half_view)
        } else {
            (half_view, half_view / aspect)
        };

        let pixel_size = (half_width * 2.0) / f64::from(hsize);

        Ok(Self {
            hsize,
            vsize,
            field_of_view,
            pixel_size,
            half_height,
            half_width,
            transform,
            transform_inverse: transform.inverse(),
        })
    }

    pub fn render(&self, world: &World, progress: RenderProgress) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let mutex = Arc::new(Mutex::new(&mut image));

        let threads: usize = std::env::var("RENDER_THREADS").map_or(DEFAULT_THREADS, |value| {
            value.parse().unwrap_or(DEFAULT_THREADS)
        });

        // https://docs.rs/rayon/1.6.1/rayon/struct.ThreadPoolBuildError.html
        #[allow(clippy::unwrap_used)]
        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        let progress_bar = match progress {
            RenderProgress::Enable => ProgressBar::new((self.hsize * self.vsize) as u64),
            RenderProgress::Disable => ProgressBar::hidden(),
        };

        pool.scope(|s| {
            for y in 0..self.vsize {
                let image = Arc::clone(&mutex);
                let progress_bar = progress_bar.clone();

                s.spawn(move |_| {
                    let mut buffer = Vec::with_capacity(self.hsize as usize);

                    for x in 0..self.hsize {
                        let ray = self.ray_for_pixel(x, y);
                        let color = world.color_at(&ray, crate::world::RECURSION_DEPTH);
                        buffer.push((x, color));

                        progress_bar.inc(1);
                    }

                    // https://doc.rust-lang.org/std/sync/type.LockResult.html
                    #[allow(clippy::unwrap_used)]
                    let mut image = image.lock().unwrap();
                    for (x, pixel) in buffer {
                        image.write_pixel(x, y, pixel);
                    }
                });
            }
        });

        image
    }

    fn ray_for_pixel(&self, x: u32, y: u32) -> Ray {
        let xoffset = (f64::from(x) + 0.5) * self.pixel_size;
        let yoffset = (f64::from(y) + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform_inverse * Point::new(world_x, world_y, -1.0);
        let origin = self.transform_inverse * Point::new(0.0, 0.0, 0.0);

        // The transformation is isomorphic, therefore `pixel` and `origin` are always going to be
        // different points because `Point::new(... -1)` is always different to `Point::new(... 0)`.
        #[allow(clippy::unwrap_used)]
        let direction = (pixel - origin).normalize().unwrap();

        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_approx, color::Color, tuple::Vector, world::test_world};

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f64::consts::FRAC_PI_2;

        let c = Camera::try_new(hsize, vsize, field_of_view, Default::default()).unwrap();

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_approx!(c.field_of_view, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.transform, Transform::default());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::try_new(200, 125, std::f64::consts::FRAC_PI_2, Default::default()).unwrap();

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::try_new(125, 200, std::f64::consts::FRAC_PI_2, Default::default()).unwrap();

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::try_new(201, 101, std::f64::consts::FRAC_PI_2, Default::default()).unwrap();

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::try_new(201, 101, std::f64::consts::FRAC_PI_2, Default::default()).unwrap();

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::try_new(
            201,
            101,
            std::f64::consts::FRAC_PI_2,
            Transform::rotation_y(std::f64::consts::FRAC_PI_4)
                * Transform::translation(0.0, -2.0, 5.0),
        )
        .unwrap();

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2_f64.sqrt() / 2.0, 0.0, -2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = test_world();

        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let c = Camera::try_new(
            11,
            11,
            std::f64::consts::FRAC_PI_2,
            Transform::try_view(from, to, up).unwrap(),
        )
        .unwrap();

        let image = c.render(&w, RenderProgress::Disable);

        assert_eq!(
            image.pixel_at(5, 5),
            &Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855,
            }
        );
    }

    #[test]
    fn comparing_cameras() {
        let c0 =
            Camera::try_new(100, 200, std::f64::consts::FRAC_PI_3, Default::default()).unwrap();
        let c1 =
            Camera::try_new(100, 200, std::f64::consts::FRAC_PI_3, Default::default()).unwrap();

        let c2 = Camera::try_new(
            200,
            300,
            std::f64::consts::FRAC_PI_6,
            Transform::try_scaling(1.0, 2.0, 3.0).unwrap(),
        )
        .unwrap();

        assert_eq!(c0, c1);
        assert_ne!(c0, c2);
    }

    #[test]
    fn trying_to_create_a_camera_with_null_dimensions() {
        let c = Camera::try_new(0, 0, std::f64::consts::FRAC_PI_2, Default::default());

        assert_eq!(c, Err(CameraError::NullDimension));
    }

    #[test]
    fn trying_to_create_a_camera_with_a_multiple_of_pi_field_of_view() {
        let c0 = Camera::try_new(100, 200, 0.0, Default::default());
        let c1 = Camera::try_new(100, 200, std::f64::consts::PI, Default::default());
        let c2 = Camera::try_new(100, 200, 3.0 * std::f64::consts::PI, Default::default());

        assert_eq!(c0, Err(CameraError::MultipleOfPiFieldOfView));
        assert_eq!(c1, Err(CameraError::MultipleOfPiFieldOfView));
        assert_eq!(c2, Err(CameraError::MultipleOfPiFieldOfView));
    }
}
