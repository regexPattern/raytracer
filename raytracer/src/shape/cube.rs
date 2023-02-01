use crate::{
    float,
    intersection::Intersection,
    ray::Ray,
    tuple::{Point, Tuple, Vector},
};

use super::{bounding_box::BoundingBox, object::ObjectCache, Shape, ShapeBuilder};

/// Representation of a cube.
///
/// Must be built from a [ShapeBuilder].
#[derive(Clone, Debug, PartialEq)]
pub struct Cube(pub(crate) ObjectCache);

impl Default for Cube {
    fn default() -> Self {
        Self::from(ShapeBuilder::default())
    }
}

impl From<ShapeBuilder> for Cube {
    fn from(builder: ShapeBuilder) -> Self {
        let ShapeBuilder {
            material,
            transform,
        } = builder;

        let bounding_box = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        Self(ObjectCache::new(material, transform, bounding_box))
    }
}

/// Intersect a ray with a rectangular bounding box.
pub fn intersect_box_with_bouding_box<'a>(ray: &Ray, bounding_box: &BoundingBox) -> (f64, f64) {
    let (xtmin, xtmax) = check_axis(
        ray.origin.0.x,
        ray.direction.0.x,
        bounding_box.min.0.x,
        bounding_box.max.0.x,
    );

    let (ytmin, ytmax) = check_axis(
        ray.origin.0.y,
        ray.direction.0.y,
        bounding_box.min.0.y,
        bounding_box.max.0.y,
    );

    let (ztmin, ztmax) = check_axis(
        ray.origin.0.z,
        ray.direction.0.z,
        bounding_box.min.0.z,
        bounding_box.max.0.z,
    );

    // There's always going to be a minimum value among these.
    #[allow(clippy::unwrap_used)]
    let tmin = [xtmin, ytmin, ztmin]
        .into_iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    // Similarly there's always going to be a maximum value among these.
    #[allow(clippy::unwrap_used)]
    let tmax = [xtmax, ytmax, ztmax]
        .into_iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    (tmin, tmax)
}

/// Check if a point lays between the `min` and `max` values in an axis.
fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
    let tmin_numerator = min - origin;
    let tmax_numerator = max - origin;

    let (tmin, tmax) = if float::ge(direction.abs(), float::EPSILON) {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (
            tmin_numerator * std::f64::INFINITY,
            tmax_numerator * std::f64::INFINITY,
        )
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl Cube {
    /// Computes a cube's local intersections.
    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
        let (tmin, tmax) = intersect_box_with_bouding_box(ray, &self.0.bounding_box);

        if tmin > tmax {
            vec![]
        } else {
            vec![
                Intersection {
                    t: tmin,
                    object,
                    u: None,
                    v: None,
                },
                Intersection {
                    t: tmax,
                    object,
                    u: None,
                    v: None,
                },
            ]
        }
    }

    /// Computes a cube's normal at a given point.
    pub(crate) fn normal_at(&self, point: Point) -> Vector {
        let Point(Tuple { x, y, z, .. }) = point;

        let max_coord = [x.abs(), y.abs(), z.abs()]
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .cloned()
            .unwrap();

        if float::approx(max_coord, x.abs()) {
            Vector::new(x, 0.0, 0.0)
        } else if float::approx(max_coord, y.abs()) {
            Vector::new(0.0, y, 0.0)
        } else {
            Vector::new(0.0, 0.0, z)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube_from_the_x_axis() {
        let cube = Cube::default();
        let object = Shape::Cube(Default::default());

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(5.0, 0.5, 0.0),
                direction: Vector::new(-1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(-5.0, 0.5, 0.0),
                direction: Vector::new(1.0, 0.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_y_axis() {
        let cube = Cube::default();
        let object = Shape::Cube(Default::default());

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(0.5, 5.0, 0.0),
                direction: Vector::new(0.0, -1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(0.5, -5.0, 0.0),
                direction: Vector::new(0.0, 1.0, 0.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_z_axis() {
        let cube = Cube::default();
        let object = Shape::Cube(Default::default());

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(0.5, 0.0, 5.0),
                direction: Vector::new(0.0, 0.0, -1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);

        let xs = cube.intersect(
            &object,
            &Ray {
                origin: Point::new(0.5, 0.0, -5.0),
                direction: Vector::new(0.0, 0.0, 1.0),
            },
        );

        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_intersects_a_cube_from_the_inside() {
        let cube = Cube::default();
        let object = Shape::Cube(Default::default());

        let r = &Ray {
            origin: Point::new(0.0, 0.5, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = cube.intersect(&object, r);

        assert_approx!(xs[0].t, -1.0);
        assert_approx!(xs[1].t, 1.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let cube = Cube::default();
        let object = Shape::Cube(Default::default());

        assert!(cube
            .intersect(
                &object,
                &Ray {
                    origin: Point::new(-2.0, 0.0, 0.0),
                    direction: Vector::new(0.2673, 0.5345, 0.8018),
                },
            )
            .is_empty());

        assert!(cube
            .intersect(
                &object,
                &Ray {
                    origin: Point::new(0.0, -2.0, 0.0),
                    direction: Vector::new(0.8018, 0.2673, 0.5345),
                },
            )
            .is_empty());

        assert!(cube
            .intersect(
                &object,
                &Ray {
                    origin: Point::new(0.0, 0.0, -2.0),
                    direction: Vector::new(0.5345, 0.8018, 0.2673),
                },
            )
            .is_empty());

        assert!(cube
            .intersect(
                &object,
                &Ray {
                    origin: Point::new(2.0, 0.0, 2.0),
                    direction: Vector::new(0.0, 0.0, -1.0)
                },
            )
            .is_empty());

        assert_eq!(
            cube.intersect(
                &object,
                &Ray {
                    origin: Point::new(0.0, 2.0, 2.0),
                    direction: Vector::new(0.0, -1.0, 0.0)
                },
            )
            .len(),
            0
        );

        assert_eq!(
            cube.intersect(
                &object,
                &Ray {
                    origin: Point::new(2.0, 2.0, 0.0),
                    direction: Vector::new(-1.0, 0.0, 0.0)
                },
            )
            .len(),
            0
        );
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let cube = Cube::default();

        assert_eq!(
            cube.normal_at(Point::new(1.0, 0.5, -0.8)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(-1.0, -0.2, 0.9)),
            Vector::new(-1.0, 0.0, 0.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(-0.4, 1.0, -0.1)),
            Vector::new(0.0, 1.0, 0.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(0.3, -1.0, -0.7)),
            Vector::new(0.0, -1.0, 0.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(-0.6, 0.3, 1.0)),
            Vector::new(0.0, 0.0, 1.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(0.4, 0.4, -1.0)),
            Vector::new(0.0, 0.0, -1.0)
        );
    }

    #[test]
    fn the_normal_on_the_corners_of_a_cube() {
        let cube = Cube::default();

        assert_eq!(
            cube.normal_at(Point::new(1.0, 1.0, 1.0)),
            Vector::new(1.0, 0.0, 0.0)
        );

        assert_eq!(
            cube.normal_at(Point::new(-1.0, -1.0, -1.0)),
            Vector::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn a_cube_has_a_bounding_box() {
        let cube = Cube::default();
        let bounding_box = cube.0.bounding_box;

        assert_eq!(bounding_box.min, Point::new(-1.0, -1.0, -1.0));
        assert_eq!(bounding_box.max, Point::new(1.0, 1.0, 1.0));
    }
}
