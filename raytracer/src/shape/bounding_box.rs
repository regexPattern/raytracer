use crate::{ray::Ray, transform::Transform, tuple::Point};

use super::{cube, Shape};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min: Point::new(std::f64::INFINITY, std::f64::INFINITY, std::f64::INFINITY),
            max: Point::new(
                std::f64::NEG_INFINITY,
                std::f64::NEG_INFINITY,
                std::f64::NEG_INFINITY,
            ),
        }
    }
}

impl<T> From<T> for BoundingBox
where
    T: IntoIterator<Item = Point>,
{
    fn from(value: T) -> Self {
        let mut bounding_box = Self::default();
        for point in value {
            bounding_box.add(point);
        }

        bounding_box
    }
}

impl BoundingBox {
    pub fn add(&mut self, point: Point) {
        self.min.0.x = f64::min(point.0.x, self.min.0.x);
        self.min.0.y = f64::min(point.0.y, self.min.0.y);
        self.min.0.z = f64::min(point.0.z, self.min.0.z);

        self.max.0.x = f64::max(point.0.x, self.max.0.x);
        self.max.0.y = f64::max(point.0.y, self.max.0.y);
        self.max.0.z = f64::max(point.0.z, self.max.0.z);
    }

    pub fn merge(&mut self, rhs: Self) {
        self.add(rhs.min);
        self.add(rhs.max);
    }

    pub fn contains_point(&self, point: Point) -> bool {
        is_between_range(point.0.x, self.min.0.x, self.max.0.x)
            && is_between_range(point.0.y, self.min.0.y, self.max.0.y)
            && is_between_range(point.0.z, self.min.0.z, self.max.0.z)
    }

    pub fn contains(&self, other: &BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    pub fn transform(self, transform: Transform) -> Self {
        let corners = [
            self.min,
            Point::new(self.min.0.x, self.min.0.y, self.max.0.z),
            Point::new(self.min.0.x, self.max.0.y, self.min.0.z),
            Point::new(self.min.0.x, self.max.0.y, self.max.0.z),
            Point::new(self.max.0.x, self.min.0.y, self.min.0.z),
            Point::new(self.max.0.x, self.min.0.y, self.max.0.z),
            Point::new(self.max.0.x, self.max.0.y, self.min.0.z),
            self.max,
        ]
        .into_iter()
        .map(|point| transform * point);

        BoundingBox::from(corners)
    }

    pub fn intersect(&self, ray: &Ray) -> bool {
        !cube::intersect_box_with_bounds(&Shape::Cube(Default::default()), ray, self).is_empty()
    }

    pub fn split(&self) -> (Self, Self) {
        use crate::{float, tuple::Tuple};

        let dx = (self.min.0.x - self.max.0.x).abs();
        let dy = (self.min.0.y - self.max.0.y).abs();
        let dz = (self.min.0.z - self.max.0.z).abs();

        // There's always going to be a largest_axis, in case all three axis are the same there is
        // still going to be a valid axis. No geometric figure except for planes, has all of it's
        // axis with infinite length. In the case of planes bounding boxes should have infinite
        // length, so infinite would count as a valid largest_axis value.
        #[allow(clippy::unwrap_used)]
        let largest_axis = [dx, dy, dz]
            .into_iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let Point(Tuple {
            x: mut x0,
            y: mut y0,
            z: mut z0,
            ..
        }) = self.min;

        let Point(Tuple {
            x: mut x1,
            y: mut y1,
            z: mut z1,
            ..
        }) = self.max;

        if float::approx(largest_axis, dx) {
            let tmp = x0 + dx / 2.0;
            x0 = tmp;
            x1 = tmp;
        } else if float::approx(largest_axis, dy) {
            let tmp = y0 + dy / 2.0;
            y0 = tmp;
            y1 = tmp;
        } else {
            let tmp = z0 + dz / 2.0;
            z0 = tmp;
            z1 = tmp;
        }

        let left = BoundingBox {
            min: self.min,
            max: Point::new(x1, y1, z1),
        };

        let right = BoundingBox {
            min: Point::new(x0, y0, z0),
            max: self.max,
        };

        (left, right)
    }
}

fn is_between_range(x: f64, lower: f64, greater: f64) -> bool {
    crate::float::ge(x, lower) && crate::float::le(x, greater)
}

#[cfg(test)]
mod tests {
    use crate::tuple::Vector;

    use super::*;

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let mut bounds = BoundingBox::default();
        let p0 = Point::new(-5.0, 2.0, 0.0);
        let p1 = Point::new(7.0, 0.0, -3.0);

        bounds.add(p0);
        bounds.add(p1);

        assert_eq!(bounds.min, Point::new(-5.0, 0.0, -3.0));
        assert_eq!(bounds.max, Point::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_one_bouding_box_to_another() {
        let mut bounds = BoundingBox {
            min: Point::new(-5.0, -2.0, 0.0),
            max: Point::new(7.0, 4.0, 4.0),
        };

        let bounds1 = BoundingBox {
            min: Point::new(8.0, -7.0, -2.0),
            max: Point::new(14.0, 2.0, 8.0),
        };

        bounds.merge(bounds1);

        assert_eq!(bounds.min, Point::new(-5.0, -7.0, -2.0));
        assert_eq!(bounds.max, Point::new(14.0, 4.0, 8.0));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_point() {
        let bounds = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bounds.contains_point(Point::new(5.0, -2.0, 0.0)));
        assert!(bounds.contains_point(Point::new(11.0, 4.0, 7.0)));
        assert!(bounds.contains_point(Point::new(8.0, 1.0, 3.0)));
        assert!(!bounds.contains_point(Point::new(3.0, 0.0, 3.0)));
        assert!(!bounds.contains_point(Point::new(8.0, -4.0, 3.0)));
        assert!(!bounds.contains_point(Point::new(8.0, 1.0, -1.0)));
        assert!(!bounds.contains_point(Point::new(13.0, 1.0, 3.0)));
        assert!(!bounds.contains_point(Point::new(8.0, 5.0, 3.0)));
        assert!(!bounds.contains_point(Point::new(8.0, 1.0, 8.0)));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_another_box() {
        let bounds = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bounds.contains(&BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0)
        }));

        assert!(bounds.contains(&BoundingBox {
            min: Point::new(6.0, -1.0, 1.0),
            max: Point::new(10.0, 3.0, 6.0)
        }));

        assert!(!bounds.contains(&BoundingBox {
            min: Point::new(4.0, -3.0, -1.0),
            max: Point::new(10.0, 3.0, 6.0)
        }));

        assert!(!bounds.contains(&BoundingBox {
            min: Point::new(6.0, -1.0, 1.0),
            max: Point::new(12.0, 5.0, 8.0)
        }));
    }

    #[test]
    fn transforming_a_bounding_box() {
        let bounds0 = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        let t = Transform::rotation_x(std::f64::consts::FRAC_PI_4)
            * Transform::rotation_y(std::f64::consts::FRAC_PI_4);

        let bounds1 = bounds0.transform(t);

        assert_eq!(bounds1.min, Point::new(-1.41421, -1.7071, -1.7071));
        assert_eq!(bounds1.max, Point::new(1.41421, 1.7071, 1.7071));
    }

    #[test]
    fn intersecting_a_ray_with_a_bouding_box_at_the_origin() {
        let bounds = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        assert!(bounds.intersect(&Ray {
            origin: Point::new(5.0, 0.5, 0.0),
            direction: Vector::new(-1.0, 0.0, 0.0),
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(-5.0, 0.5, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(0.5, 5.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(0.5, -5.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(0.5, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(0.5, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(0.0, 0.5, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(-2.0, 0.0, 0.0),
            direction: Vector::new(2.0, 4.0, 6.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(0.0, -2.0, 0.0),
            direction: Vector::new(6.0, 2.0, 4.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(0.0, 0.0, -2.0),
            direction: Vector::new(4.0, 6.0, 2.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(2.0, 0.0, 2.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(0.0, 2.0, 2.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(2.0, 2.0, 0.0),
            direction: Vector::new(-1.0, 0.0, 0.0)
        }));
    }

    #[test]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let bounds = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bounds.intersect(&Ray {
            origin: Point::new(5.0, 1.0, 2.0),
            direction: Vector::new(1.0, 0.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(-5.0, -1.0, 4.0),
            direction: Vector::new(1.0, 0.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(7.0, 6.0, 5.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(9.0, -5.0, 6.0),
            direction: Vector::new(0.0, 1.0, 0.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(8.0, 2.0, 12.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(6.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(bounds.intersect(&Ray {
            origin: Point::new(8.0, 1.0, 3.5),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(9.0, -1.0, -8.0),
            direction: Vector::new(2.0, 4.0, 6.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(8.0, 3.0, -4.0),
            direction: Vector::new(6.0, 2.0, 4.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(9.0, -1.0, -2.0),
            direction: Vector::new(4.0, 6.0, 2.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(4.0, 0.0, 9.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(8.0, 6.0, -1.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(!bounds.intersect(&Ray {
            origin: Point::new(12.0, 5.0, 4.0),
            direction: Vector::new(-1.0, 0.0, 0.0)
        }));
    }

    #[test]
    fn splitting_a_perfect_cube() {
        let bounds = BoundingBox {
            min: Point::new(-1.0, -4.0, -5.0),
            max: Point::new(9.0, 6.0, 5.0),
        };

        let (left, right) = bounds.split();

        assert_eq!(left.min, Point::new(-1.0, -4.0, -5.0));
        assert_eq!(left.max, Point::new(4.0, 6.0, 5.0));

        assert_eq!(right.min, Point::new(4.0, -4.0, -5.0));
        assert_eq!(right.max, Point::new(9.0, 6.0, 5.0));
    }

    #[test]
    fn splitting_an_x_wide_bounding_box() {
        let bounds = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(9.0, 5.5, 3.0),
        };

        let (left, right) = bounds.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(4.0, 5.5, 3.0));

        assert_eq!(right.min, Point::new(4.0, -2.0, -3.0));
        assert_eq!(right.max, Point::new(9.0, 5.5, 3.0));
    }

    #[test]
    fn splitting_an_y_wide_bounding_box() {
        let bounds = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(5.0, 8.0, 3.0),
        };

        let (left, right) = bounds.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 3.0));

        assert_eq!(right.min, Point::new(-1.0, 3.0, -3.0));
        assert_eq!(right.max, Point::new(5.0, 8.0, 3.0));
    }

    #[test]
    fn splitting_an_z_wide_bounding_box() {
        let bounds = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(5.0, 3.0, 7.0),
        };

        let (left, right) = bounds.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 2.0));

        assert_eq!(right.min, Point::new(-1.0, -2.0, 2.0));
        assert_eq!(right.max, Point::new(5.0, 3.0, 7.0));
    }
}
