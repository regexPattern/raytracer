use crate::{ray::Ray, transform::Transform, tuple::Point};

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
        let mut bbox = Self::default();

        for point in value {
            bbox.add(point);
        }

        bbox
    }
}

fn is_between_range(x: f64, lower: f64, greater: f64) -> bool {
    crate::float::ge(x, lower) && crate::float::le(x, greater)
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

    pub fn contains_box(&self, other: &BoundingBox) -> bool {
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
        use crate::shape::{cube, Shape};
        !cube::intersect(&Shape::Cube(Default::default()), ray, self).is_empty()
    }

    pub fn split(&self) -> (Self, Self) {
        use crate::{float, tuple::Tuple};

        let dim_x = self.min.0.x.abs() + self.max.0.x.abs();
        let dim_y = self.min.0.y.abs() + self.max.0.y.abs();
        let dim_z = self.min.0.z.abs() + self.max.0.z.abs();

        // There's always going to be a largest_axis, in case all three axis are the same there is
        // still going to be a valid axis. No geometric figure except for planes, has all of it's
        // axis with infinite length. In the case of planes bounding boxes should have infinite
        // length, so infinite would count as a valid largest_axis value.
        #[allow(clippy::unwrap_used)]
        let largest_axis = [dim_x, dim_y, dim_z]
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

        if float::approx(largest_axis, dim_x) {
            let tmp = x0 + dim_x / 2.0;
            x0 = tmp;
            x1 = tmp;
        } else if float::approx(largest_axis, dim_y) {
            let tmp = y0 + dim_y / 2.0;
            y0 = tmp;
            y1 = tmp;
        } else {
            let tmp = z0 + dim_z / 2.0;
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

#[cfg(test)]
mod tests {
    use crate::tuple::Vector;

    use super::*;

    #[test]
    fn adding_points_to_an_empty_bounding_box() {
        let mut bbox = BoundingBox::default();
        let p1 = Point::new(-5.0, 2.0, 0.0);
        let p2 = Point::new(7.0, 0.0, -3.0);

        bbox.add(p1);
        bbox.add(p2);

        assert_eq!(bbox.min, Point::new(-5.0, 0.0, -3.0));
        assert_eq!(bbox.max, Point::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_one_bouding_box_to_another() {
        let mut bbox0 = BoundingBox {
            min: Point::new(-5.0, -2.0, 0.0),
            max: Point::new(7.0, 4.0, 4.0),
        };

        let bbox1 = BoundingBox {
            min: Point::new(8.0, -7.0, -2.0),
            max: Point::new(14.0, 2.0, 8.0),
        };

        bbox0.merge(bbox1);

        assert_eq!(bbox0.min, Point::new(-5.0, -7.0, -2.0));
        assert_eq!(bbox0.max, Point::new(14.0, 4.0, 8.0));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_a_given_point() {
        let bbox = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bbox.contains_point(Point::new(5.0, -2.0, 0.0)));
        assert!(bbox.contains_point(Point::new(11.0, 4.0, 7.0)));
        assert!(bbox.contains_point(Point::new(8.0, 1.0, 3.0)));
        assert!(!bbox.contains_point(Point::new(3.0, 0.0, 3.0)));
        assert!(!bbox.contains_point(Point::new(8.0, -4.0, 3.0)));
        assert!(!bbox.contains_point(Point::new(8.0, 1.0, -1.0)));
        assert!(!bbox.contains_point(Point::new(13.0, 1.0, 3.0)));
        assert!(!bbox.contains_point(Point::new(8.0, 5.0, 3.0)));
        assert!(!bbox.contains_point(Point::new(8.0, 1.0, 8.0)));
    }

    #[test]
    fn checking_to_see_if_a_box_contains_another_box() {
        let bbox = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bbox.contains_box(&BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0)
        }));

        assert!(bbox.contains_box(&BoundingBox {
            min: Point::new(6.0, -1.0, 1.0),
            max: Point::new(10.0, 3.0, 6.0)
        }));

        assert!(!bbox.contains_box(&BoundingBox {
            min: Point::new(4.0, -3.0, -1.0),
            max: Point::new(10.0, 3.0, 6.0)
        }));

        assert!(!bbox.contains_box(&BoundingBox {
            min: Point::new(6.0, -1.0, 1.0),
            max: Point::new(12.0, 5.0, 8.0)
        }));
    }

    #[test]
    fn transforming_a_bounding_box() {
        let bbox0 = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        let t = Transform::rotation_x(std::f64::consts::FRAC_PI_4)
            * Transform::rotation_y(std::f64::consts::FRAC_PI_4);

        let bbox1 = bbox0.transform(t);

        assert_eq!(bbox1.min, Point::new(-1.41421, -1.7071, -1.7071));
        assert_eq!(bbox1.max, Point::new(1.41421, 1.7071, 1.7071));
    }

    #[test]
    fn intersecting_a_ray_with_a_bouding_box_at_the_origin() {
        let bbox = BoundingBox {
            min: Point::new(-1.0, -1.0, -1.0),
            max: Point::new(1.0, 1.0, 1.0),
        };

        assert!(bbox.intersect(&Ray {
            origin: Point::new(5.0, 0.5, 0.0),
            direction: Vector::new(-1.0, 0.0, 0.0),
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(-5.0, 0.5, 0.0),
            direction: Vector::new(1.0, 0.0, 0.0),
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(0.5, 5.0, 0.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(0.5, -5.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(0.5, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(0.5, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(0.0, 0.5, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(-2.0, 0.0, 0.0),
            direction: Vector::new(2.0, 4.0, 6.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(0.0, -2.0, 0.0),
            direction: Vector::new(6.0, 2.0, 4.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(0.0, 0.0, -2.0),
            direction: Vector::new(4.0, 6.0, 2.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(2.0, 0.0, 2.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(0.0, 2.0, 2.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(2.0, 2.0, 0.0),
            direction: Vector::new(-1.0, 0.0, 0.0)
        }));
    }

    #[test]
    fn intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let bbox = BoundingBox {
            min: Point::new(5.0, -2.0, 0.0),
            max: Point::new(11.0, 4.0, 7.0),
        };

        assert!(bbox.intersect(&Ray {
            origin: Point::new(5.0, 1.0, 2.0),
            direction: Vector::new(1.0, 0.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(-5.0, -1.0, 4.0),
            direction: Vector::new(1.0, 0.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(7.0, 6.0, 5.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(9.0, -5.0, 6.0),
            direction: Vector::new(0.0, 1.0, 0.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(8.0, 2.0, 12.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(6.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(bbox.intersect(&Ray {
            origin: Point::new(8.0, 1.0, 3.5),
            direction: Vector::new(0.0, 0.0, 1.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(9.0, -1.0, -8.0),
            direction: Vector::new(2.0, 4.0, 6.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(8.0, 3.0, -4.0),
            direction: Vector::new(6.0, 2.0, 4.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(9.0, -1.0, -2.0),
            direction: Vector::new(4.0, 6.0, 2.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(4.0, 0.0, 9.0),
            direction: Vector::new(0.0, 0.0, -1.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(8.0, 6.0, -1.0),
            direction: Vector::new(0.0, -1.0, 0.0)
        }));

        assert!(!bbox.intersect(&Ray {
            origin: Point::new(12.0, 5.0, 4.0),
            direction: Vector::new(-1.0, 0.0, 0.0)
        }));
    }

    #[test]
    fn splitting_a_perfect_cube() {
        let bbox = BoundingBox {
            min: Point::new(-1.0, -4.0, -5.0),
            max: Point::new(9.0, 6.0, 5.0),
        };

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -4.0, -5.0));
        assert_eq!(left.max, Point::new(4.0, 6.0, 5.0));

        assert_eq!(right.min, Point::new(4.0, -4.0, -5.0));
        assert_eq!(right.max, Point::new(9.0, 6.0, 5.0));
    }

    #[test]
    fn splitting_an_x_wide_bounding_box() {
        let bbox = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(9.0, 5.5, 3.0),
        };

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(4.0, 5.5, 3.0));

        assert_eq!(right.min, Point::new(4.0, -2.0, -3.0));
        assert_eq!(right.max, Point::new(9.0, 5.5, 3.0));
    }

    #[test]
    fn splitting_an_y_wide_bounding_box() {
        let bbox = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(5.0, 8.0, 3.0),
        };

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 3.0));

        assert_eq!(right.min, Point::new(-1.0, 3.0, -3.0));
        assert_eq!(right.max, Point::new(5.0, 8.0, 3.0));
    }

    #[test]
    fn splitting_an_z_wide_bounding_box() {
        let bbox = BoundingBox {
            min: Point::new(-1.0, -2.0, -3.0),
            max: Point::new(5.0, 3.0, 7.0),
        };

        let (left, right) = bbox.split();

        assert_eq!(left.min, Point::new(-1.0, -2.0, -3.0));
        assert_eq!(left.max, Point::new(5.0, 3.0, 2.0));

        assert_eq!(right.min, Point::new(-1.0, -2.0, 2.0));
        assert_eq!(right.max, Point::new(5.0, 3.0, 7.0));
    }
}
