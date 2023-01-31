use rand::Rng;

use crate::{
    color::Color,
    tuple::{Point, Vector},
    world::World,
};

#[derive(Debug)]
pub enum Light {
    Area(AreaLight),
    Point(PointLight),
}

#[derive(Copy, Clone, Debug)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[derive(Debug)]
pub struct AreaLight {
    corner: Point,
    uvec: Vector,
    usteps: usize,
    vvec: Vector,
    vsteps: usize,
    pub(crate) samples: usize,
    intensity: Color,
}

pub struct AreaLightBuilder {
    pub corner: Point,
    pub horizontal_vec: Vector,
    pub horizontal_cells: usize,
    pub vertical_vec: Vector,
    pub vertical_cells: usize,
    pub intensity: Color,
}

impl From<AreaLightBuilder> for AreaLight {
    fn from(builder: AreaLightBuilder) -> Self {
        let AreaLightBuilder {
            corner,
            horizontal_vec,
            horizontal_cells: usteps,
            vertical_vec,
            vertical_cells: vsteps,
            intensity,
        } = builder;

        // TODO: Handle this unwrap.
        let uvec = (horizontal_vec / usteps as f64).unwrap();
        let vvec = (vertical_vec / vsteps as f64).unwrap();

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
    pub(crate) fn intensity_at(&self, world: &World, point: Point) -> f64 {
        match self {
            Self::Area(area_light) => area_light.intensity_at(world, point, || {
                let mut rng = rand::thread_rng();
                rng.gen::<u8>() as f64 / 255.0
            }),
            Self::Point(point_light) => point_light.intensity_at(world, point),
        }
    }

    pub(crate) fn samples(&self) -> Vec<Point> {
        match self {
            Self::Area(area_light) => {
                let mut samples = vec![];
                for v in 0..area_light.vsteps {
                    for u in 0..area_light.usteps {
                        samples.push(area_light.point_on_light(u, v, || 0.5));
                    }
                }

                samples
            }
            Self::Point(point_light) => vec![point_light.position],
        }
    }

    pub(crate) fn intensity(&self) -> Color {
        match self {
            Self::Area(area_light) => area_light.intensity,
            Self::Point(point_light) => point_light.intensity,
        }
    }
}

impl PointLight {
    fn intensity_at(&self, world: &World, point: Point) -> f64 {
        if world.is_shadowed(self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

impl AreaLight {
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
            horizontal_vec,
            horizontal_cells: 4,
            vertical_vec,
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
            horizontal_vec,
            horizontal_cells: 4,
            vertical_vec,
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
            horizontal_vec,
            horizontal_cells: 2,
            vertical_vec,
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
            horizontal_vec,
            horizontal_cells: 4,
            vertical_vec,
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
