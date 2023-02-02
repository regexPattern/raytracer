use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex},
};

use indicatif::ProgressBar;
use rayon::ThreadPoolBuilder;
use thiserror::Error;

use crate::{canvas::Canvas, float, ray::Ray, transform::Transform, tuple::Point, world::World};

/// Module constants.
pub mod consts;

/// Default number of threads using during the world-rendering process.
const DEFAULT_RENDER_THREADS: usize = 8;

/// The error type when trying to create a camera.
///
/// Errors originate from the values of the [CameraBuilder] used to construct a camera.
///
#[derive(Debug, PartialEq, Error)]
pub enum Error {
    /// The error type when trying to create a camera with no dimensions.
    #[error("camera cannot have null dimensions")]
    NullDimension,

    /// The error type when trying to create a camera with a field of view that is a multiple of
    /// pi. This is an invalid value because it would create a camera with an infinitely wide
    /// viewport.
    ///
    #[error("field of view angle cannot be straight")]
    MultipleOfPiFieldOfView,
}

/// Viewport into a scene.
///
/// Cameras are used a to "take a picture" of a world.
///
/// # Examples
///
/// A camera must be built from a [CameraBuilder].
///
/// ```
/// use raytracer::{
///     camera::{Camera, CameraBuilder},
///     transform::Transform,
///     tuple::{Point, Vector},
/// };
///
/// let camera = Camera::try_from(CameraBuilder {
///     width: 1920,
///     height: 1080,
///     field_of_view: std::f64::consts::FRAC_PI_3,
///     transform: Transform::view(
///         Point::new(0.0, 5.0, 5.0),
///         Point::new(0.0, 1.0, 0.0),
///         Vector::new(0.0, 1.0, 0.0),
///     ).unwrap(),
/// }).unwrap();
/// ```
///
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    pixel_size: f64,
    half_height: f64,
    half_width: f64,
    transform: Transform,
    transform_inverse: Transform,
}

/// Builder for a camera.
#[derive(Copy, Clone, Debug)]
pub struct CameraBuilder {
    /// Image width in number of pixels.
    pub width: usize,

    /// Image height in number of pixels.
    pub height: usize,

    /// Field of view for the camera's "virtual lens".
    pub field_of_view: f64,

    /// Transformation that describes the camera positioning in the world.
    ///
    /// When using a transformation other than [Transform::view], you can think of the coordinates
    /// of that transformation as being mirrored in the `xz` plane.
    ///
    pub transform: Transform,
}

impl TryFrom<CameraBuilder> for Camera {
    type Error = Error;

    fn try_from(builder: CameraBuilder) -> Result<Self, Self::Error> {
        let CameraBuilder {
            width: hsize,
            height: vsize,
            field_of_view,
            transform,
        } = builder;

        if float::approx(field_of_view % std::f64::consts::PI, 0.0) {
            return Err(Error::MultipleOfPiFieldOfView);
        }

        let hsize = NonZeroUsize::new(hsize).ok_or(Error::NullDimension)?.get();

        let vsize = NonZeroUsize::new(vsize).ok_or(Error::NullDimension)?.get();

        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect < 1.0 {
            (half_view * aspect, half_view)
        } else {
            (half_view, half_view / aspect)
        };

        let pixel_size = (half_width * 2.0) / hsize as f64;

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
    /// Renders the given world using the camera.
    ///
    /// The rendering process is multithreaded by default, using a thread-pool with a default
    /// number of threads. This value can be overridden passing the environment variable
    /// `RENDER_THREADS` with the desired number of threads.
    ///
    /// # Panics:
    ///
    /// * If [ThreadPoolBuilder::build](https://docs.rs/rayon/latest/rayon/struct.ThreadPoolBuilder.html#method.build) fails.
    /// * If [Mutex::lock](https://doc.rust-lang.org/std/sync/struct.Mutex.html#method.lock) fails.
    ///
    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);
        let mutex = Arc::new(Mutex::new(&mut image));

        let threads: usize = std::env::var("RENDER_THREADS")
            .map_or(DEFAULT_RENDER_THREADS, |value| {
                value.parse().unwrap_or(DEFAULT_RENDER_THREADS)
            });

        let pool = ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .unwrap();

        let progress_bar = if std::env::args().any(|arg| arg == "--progress") {
            ProgressBar::new((self.hsize * self.vsize) as u64)
        } else {
            ProgressBar::hidden()
        };

        pool.scope(|s| {
            for y in 0..self.vsize {
                let image = Arc::clone(&mutex);
                let progress_bar = ProgressBar::clone(&progress_bar);

                s.spawn(move |_| {
                    let mut buffer = Vec::with_capacity(self.hsize);

                    for x in 0..self.hsize {
                        let ray = self.ray_for_pixel(x, y);
                        let color = world.color_at(&ray, crate::world::RECURSION_DEPTH);
                        buffer.push((x, color));

                        progress_bar.inc(1);
                    }

                    let mut image = image.lock().unwrap();
                    for (x, pixel) in buffer {
                        image.write_pixel(x, y, pixel);
                    }
                });
            }
        });

        image
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as f64 + 0.5) * self.pixel_size;
        let yoffset = (y as f64 + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = self.transform_inverse * Point::new(world_x, world_y, -1.0);
        let origin = self.transform_inverse * Point::new(0.0, 0.0, 0.0);

        // The transformation is ensured to be isomorphic, therefore `pixel` and `origin` are
        // always going to be different points because `Point::new(... -1)` is always different to
        // `Point::new(... 0)`.
        //
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

        let c = Camera::try_from(CameraBuilder {
            width: hsize,
            height: vsize,
            field_of_view,
            transform: Default::default(),
        })
        .unwrap();

        assert_eq!(c.hsize, hsize);
        assert_eq!(c.vsize, vsize);
        assert_approx!(c.field_of_view, std::f64::consts::FRAC_PI_2);
        assert_eq!(c.transform, Transform::default());
    }

    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::try_from(CameraBuilder {
            width: 200,
            height: 125,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Default::default(),
        })
        .unwrap();

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::try_from(CameraBuilder {
            width: 125,
            height: 200,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Default::default(),
        })
        .unwrap();

        assert_approx!(c.pixel_size, 0.01);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let c = Camera::try_from(CameraBuilder {
            width: 201,
            height: 101,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Default::default(),
        })
        .unwrap();

        let r = c.ray_for_pixel(100, 50);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::try_from(CameraBuilder {
            width: 201,
            height: 101,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Default::default(),
        })
        .unwrap();

        let r = c.ray_for_pixel(0, 0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let c = Camera::try_from(CameraBuilder {
            width: 201,
            height: 101,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Transform::rotation_y(std::f64::consts::FRAC_PI_4)
                * Transform::translation(0.0, -2.0, 5.0),
        })
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

        let c = Camera::try_from(CameraBuilder {
            width: 11,
            height: 11,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Transform::view(from, to, up).unwrap(),
        })
        .unwrap();

        let image = c.render(&w);

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
        let c0 = Camera::try_from(CameraBuilder {
            width: 100,
            height: 200,
            field_of_view: std::f64::consts::FRAC_PI_3,
            transform: Default::default(),
        })
        .unwrap();

        let c1 = Camera::try_from(CameraBuilder {
            width: 100,
            height: 200,
            field_of_view: std::f64::consts::FRAC_PI_3,
            transform: Default::default(),
        })
        .unwrap();

        let c2 = Camera::try_from(CameraBuilder {
            width: 200,
            height: 300,
            field_of_view: std::f64::consts::FRAC_PI_6,
            transform: Transform::scaling(1.0, 2.0, 3.0).unwrap(),
        })
        .unwrap();

        assert_eq!(c0, c1);
        assert_ne!(c0, c2);
    }

    #[test]
    fn trying_to_create_a_camera_with_null_dimensions() {
        let c = Camera::try_from(CameraBuilder {
            width: 0,
            height: 0,
            field_of_view: std::f64::consts::FRAC_PI_2,
            transform: Default::default(),
        });

        assert_eq!(c, Err(Error::NullDimension));
    }

    #[test]
    fn trying_to_create_a_camera_with_a_multiple_of_pi_field_of_view() {
        let c0 = Camera::try_from(CameraBuilder {
            width: 100,
            height: 200,
            field_of_view: 0.0,
            transform: Default::default(),
        });

        let c1 = Camera::try_from(CameraBuilder {
            width: 100,
            height: 200,
            field_of_view: std::f64::consts::PI,
            transform: Default::default(),
        });

        let c2 = Camera::try_from(CameraBuilder {
            width: 100,
            height: 200,
            field_of_view: 3.0 * std::f64::consts::PI,
            transform: Default::default(),
        });

        assert_eq!(c0, Err(Error::MultipleOfPiFieldOfView));
        assert_eq!(c1, Err(Error::MultipleOfPiFieldOfView));
        assert_eq!(c2, Err(Error::MultipleOfPiFieldOfView));
    }
}
