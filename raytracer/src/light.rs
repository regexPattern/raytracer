use rand::Rng;

use crate::{
    color::Color,
    tuple::{Point, Vector},
    world::World,
};

/// A world's light source.
///
/// Light are used to illumite objects in the world.
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Light {
    /// An area light.
    Area(AreaLight),

    /// A point light.
    Point(PointLight),
}

/// An infinitely-small light.
///
/// Point lights are used to create harsh shadows.
///
/// # Examples
///
/// ```
/// use raytracer::{
///     color,
///     light::{Light, PointLight},
///     tuple::Point
/// };
///
/// let light = Light::Point(PointLight {
///     position: Point::new(1.0, 1.0, 1.0),
///     intensity: color::consts::WHITE,
/// });
/// ```
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PointLight {
    /// Position of the light.
    pub position: Point,

    /// Color of the light.
    pub intensity: Color,
}

/// A rectangular grid of lights.
///
/// Area lights are used to create soft shadows.
///
/// Keep in mind that rendering soft shadows requires much more compute power than rendering
/// regular harsh shadows, specially as the number of cells in the grid grows.
///
/// # Examples
///
/// An area-light must be built from an [AreaLightBuilder].
///
/// ```
/// use raytracer::{
///     color,
///     light::{AreaLight, AreaLightBuilder, Light},
///     tuple::{Point, Vector}
/// };
///
/// // White area light with a 5x4 cells grid and the following corners:
/// // (5, 5, 5) -> (9, 5, 5) -> (9, 9, 5) -> (5, 9, 5) -> (5, 5, 5)
/// let light = Light::Area(AreaLight::from(AreaLightBuilder {
///     corner: Point::new(5.0, 5.0, 5.0),
///     horizontal_dir: Vector::new(4.0, 0.0, 0.0),
///     horizontal_cells: 5,
///     vertical_dir: Vector::new(0.0, 4.0, 0.0),
///     vertical_cells: 4,
///     intensity: color::consts::WHITE,
/// }));
/// ```
///
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AreaLight {
    corner: Point,
    uvec: Vector,
    usteps: usize,
    vvec: Vector,
    vsteps: usize,
    pub(crate) samples: usize,
    intensity: Color,
}

/// Builder for an area light.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AreaLightBuilder {
    /// Position of the bottom-left corner of the rectangular area light.
    pub corner: Point,

    /// Horizontal direction vector from the given corner to the second corner of the rectangular
    /// area's base.
    ///
    pub horizontal_dir: Vector,

    /// Number of horizontal cells in the light grid.
    pub horizontal_cells: usize,

    /// Vertical direction vector from the given corner to the second corner of the rectangular
    /// area's height.
    ///
    pub vertical_dir: Vector,

    /// Number of vertical cells in the light grid.
    pub vertical_cells: usize,

    /// Color of the light.
    pub intensity: Color,
}

impl From<AreaLightBuilder> for AreaLight {
    fn from(builder: AreaLightBuilder) -> Self {
        let AreaLightBuilder {
            corner,
            horizontal_dir,
            horizontal_cells: usteps,
            vertical_dir,
            vertical_cells: vsteps,
            intensity,
        } = builder;

        // TODO: Handle this unwrap that happens when I get null direction vectors. Also I should
        // handle the case when I receive collinear direction vectors.
        //
        let uvec = (horizontal_dir / usteps as f64).unwrap();
        let vvec = (vertical_dir / vsteps as f64).unwrap();

        Self {
            corner,
            uvec,
            usteps,
            vvec,
            vsteps,
            samples: usteps * vsteps,
            intensity,
        }
    }
}

impl Light {
    /// Returns the intensity of a light at a given point.
    pub(crate) fn intensity_at(&self, world: &World, point: Point) -> f64 {
        match self {
            Self::Area(area_light) => area_light.intensity_at(world, point, || {
                let mut rng = rand::thread_rng();
                rng.gen::<u8>() as f64 / 255.0
            }),
            Self::Point(point_light) => point_light.intensity_at(world, point),
        }
    }

    /// Returns the positions of the light cells, or the whole light if the light is a
    /// [PointLight].
    pub(crate) fn cells(&self) -> Vec<Point> {
        match self {
            Self::Area(area_light) => {
                let mut cells = vec![];
                for v in 0..area_light.vsteps {
                    for u in 0..area_light.usteps {
                        cells.push(area_light.point_on_light(u, v, || 0.5));
                    }
                }

                cells
            }
            Self::Point(point_light) => vec![point_light.position],
        }
    }

    /// Returns the effective color of a light.
    pub(crate) fn intensity(&self) -> Color {
        match self {
            Self::Area(area_light) => area_light.intensity,
            Self::Point(point_light) => point_light.intensity,
        }
    }
}

impl PointLight {
    /// Returns `0.0` if the point is in shadow. Otherwise it returns `1.0`.
    fn intensity_at(&self, world: &World, point: Point) -> f64 {
        if world.is_shadowed(self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

impl AreaLight {
    /// Returns a value between `0.0`, if the value is in
    /// [umbra](https://en.wikipedia.org/wiki/Umbra,_penumbra_and_antumbra#Umbra), and `1.0` if the
    /// value if in [antumbra](https://en.wikipedia.org/wiki/Umbra,_penumbra_and_antumbra#Umbra).
    fn intensity_at<F>(&self, world: &World, point: Point, jitter: F) -> f64
    where
        F: Fn() -> f64,
    {
        let mut total = 0.0;

        for v in 0..self.vsteps {
            for u in 0..self.usteps {
                let light_position = self.point_on_light(u, v, &jitter);

                if !world.is_shadowed(light_position, point) {
                    total += 1.0;
                }
            }
        }

        total / self.samples as f64
    }

    /// Returns a jittered position between the bounds of the corresponding light cell located at
    /// `u` width and `v` height with respect to the light corner.
    ///
    fn point_on_light<F>(&self, u: usize, v: usize, jitter: F) -> Point
    where
        F: Fn() -> f64,
    {
        self.corner + self.uvec * (u as f64 + jitter()) + self.vvec * (v as f64 + jitter())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, iter::Cycle};

    use crate::{assert_approx, color, world::test_world};

    use super::*;

    #[derive(Debug)]
    struct MockJitter<const N: usize>(Cycle<std::array::IntoIter<f64, N>>);

    impl<const N: usize> MockJitter<N> {
        fn next(&mut self) -> f64 {
            self.0.next().unwrap()
        }
    }

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = color::consts::WHITE;
        let position = Point::new(0.0, 0.0, 0.0);

        let light = PointLight {
            position,
            intensity,
        };

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn point_lights_evaluate_the_light_intensity_at_a_given_point() {
        let w = test_world();
        let light = &w.lights[0];

        assert_approx!(light.intensity_at(&w, Point::new(0.0, 1.0001, 0.0)), 1.0);
        assert_approx!(light.intensity_at(&w, Point::new(-1.0001, 0.0, 0.0)), 1.0);
        assert_approx!(light.intensity_at(&w, Point::new(0.0, 0.0, -1.0001)), 1.0);

        assert_approx!(light.intensity_at(&w, Point::new(0.0, 0.0, 1.0001)), 0.0);
        assert_approx!(light.intensity_at(&w, Point::new(1.0001, 0.0, 0.0)), 0.0);
        assert_approx!(light.intensity_at(&w, Point::new(0.0, -1.0001, 0.0)), 0.0);
        assert_approx!(light.intensity_at(&w, Point::new(0.0, 0.0, 0.0)), 0.0);
    }

    #[test]
    fn creating_an_area_light() {
        let corner = Point::new(0.0, 0.0, 0.0);
        let horizontal_vec = Vector::new(2.0, 0.0, 0.0);
        let vertical_vec = Vector::new(0.0, 0.0, 1.0);

        let light = AreaLight::from(AreaLightBuilder {
            corner,
            horizontal_dir: horizontal_vec,
            horizontal_cells: 4,
            vertical_dir: vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        });

        assert_eq!(light.corner, corner);
        assert_eq!(light.uvec, Vector::new(0.5, 0.0, 0.0));
        assert_eq!(light.usteps, 4);
        assert_eq!(light.vvec, Vector::new(0.0, 0.0, 0.5));
        assert_eq!(light.vsteps, 2);
        assert_eq!(light.samples, 8);
    }

    #[test]
    fn finding_a_single_point_on_an_area_light() {
        let corner = Point::new(0.0, 0.0, 0.0);
        let horizontal_vec = Vector::new(2.0, 0.0, 0.0);
        let vertical_vec = Vector::new(0.0, 0.0, 1.0);

        let light = AreaLight::from(AreaLightBuilder {
            corner,
            horizontal_dir: horizontal_vec,
            horizontal_cells: 4,
            vertical_dir: vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        });

        let mock_jitter = RefCell::new(MockJitter([0.5].into_iter().cycle()));
        let jitter = || mock_jitter.borrow_mut().next();

        assert_eq!(
            light.point_on_light(0, 0, jitter),
            Point::new(0.25, 0.0, 0.25)
        );

        assert_eq!(
            light.point_on_light(1, 0, jitter),
            Point::new(0.75, 0.0, 0.25)
        );

        assert_eq!(
            light.point_on_light(0, 1, jitter),
            Point::new(0.25, 0.0, 0.75)
        );

        assert_eq!(
            light.point_on_light(2, 0, jitter),
            Point::new(1.25, 0.0, 0.25)
        );

        assert_eq!(
            light.point_on_light(3, 1, jitter),
            Point::new(1.75, 0.0, 0.75)
        );
    }

    #[test]
    fn the_area_light_intensity_function() {
        let w = test_world();

        let corner = Point::new(-0.5, -0.5, -5.0);
        let horizontal_vec = Vector::new(1.0, 0.0, 0.0);
        let vertical_vec = Vector::new(0.0, 1.0, 0.0);

        let light = AreaLight::from(AreaLightBuilder {
            corner,
            horizontal_dir: horizontal_vec,
            horizontal_cells: 2,
            vertical_dir: vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        });

        let mock_jitter = RefCell::new(MockJitter([0.5].into_iter().cycle()));
        let jitter = || mock_jitter.borrow_mut().next();

        assert_approx!(
            light.intensity_at(&w, Point::new(0.0, 0.0, 2.0), jitter),
            0.0
        );

        assert_approx!(
            light.intensity_at(&w, Point::new(1.0, -1.0, 2.0), jitter),
            0.25
        );

        assert_approx!(
            light.intensity_at(&w, Point::new(1.5, 0.0, 2.0), jitter),
            0.5
        );

        assert_approx!(
            light.intensity_at(&w, Point::new(1.25, 1.25, 3.0), jitter),
            0.75
        );

        assert_approx!(
            light.intensity_at(&w, Point::new(0.0, 0.0, -2.0), jitter),
            1.0
        );
    }

    #[test]
    fn a_number_generator_returns_a_cyclic_sequence_of_numbers() {
        let mut gen = MockJitter([0.1, 0.5, 1.0].into_iter().cycle());

        assert_approx!(gen.next(), 0.1);
        assert_approx!(gen.next(), 0.5);
        assert_approx!(gen.next(), 1.0);
        assert_approx!(gen.next(), 0.1);
    }

    #[test]
    fn finding_a_single_point_on_a_jittered_area_light() {
        let corner = Point::new(0.0, 0.0, 0.0);
        let horizontal_vec = Vector::new(2.0, 0.0, 0.0);
        let vertical_vec = Vector::new(0.0, 0.0, 1.0);

        let light = AreaLight::from(AreaLightBuilder {
            corner,
            horizontal_dir: horizontal_vec,
            horizontal_cells: 4,
            vertical_dir: vertical_vec,
            vertical_cells: 2,
            intensity: color::consts::WHITE,
        });

        let mock_jitter = RefCell::new(MockJitter([0.3, 0.7].into_iter().cycle()));
        let jitter = || mock_jitter.borrow_mut().next();

        assert_eq!(
            light.point_on_light(0, 0, jitter),
            Point::new(0.15, 0.0, 0.35)
        );

        assert_eq!(
            light.point_on_light(1, 0, jitter),
            Point::new(0.65, 0.0, 0.35)
        );

        assert_eq!(
            light.point_on_light(0, 1, jitter),
            Point::new(0.15, 0.0, 0.85)
        );

        assert_eq!(
            light.point_on_light(2, 0, jitter),
            Point::new(1.15, 0.0, 0.35)
        );

        assert_eq!(
            light.point_on_light(3, 1, jitter),
            Point::new(1.65, 0.0, 0.85)
        );
    }
}
