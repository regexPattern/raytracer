use thiserror::Error;

use crate::{
    float,
    intersection::Intersection,
    material::Material,
    ray::Ray,
    tuple::{Point, Vector},
};

use super::{bounding_box::BoundingBox, object::ObjectCache, Shape};

#[derive(Debug, PartialEq, Error)]
#[error("triangle sides most not be collinear")]
pub enum Error {
    CollinearTriangleSides,
}

/// Representation of a triangle.
///
/// # Examples
///
/// A triangle must be built from a [TriangleBuilder].
///
/// ```
/// use raytracer::{
///     material::Material,
///     shape::{Shape, Triangle, TriangleBuilder},
///     tuple::Point,
/// };
///
/// let triangle = Shape::Triangle(Triangle::try_from(TriangleBuilder {
///     material: Material {
///         ambient: 0.5,
///         diffuse: 0.7,
///         specular: 0.1,
///         ..Default::default()
///     },
///     vertices: [
///         Point::new(-1.0, 0.0, 0.0),
///         Point::new(0.0, 2.0, 0.0),
///         Point::new(1.0, 0.0, 0.0),
///     ],
/// }).unwrap());
/// ```
///
#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub(crate) object_cache: ObjectCache,
    pub(crate) v0: Point,
    pub(crate) v1: Point,
    pub(crate) v2: Point,
    e0: Vector,
    e1: Vector,
    normal: Vector,
}

/// Builder for a triangle.
#[derive(Clone)]
pub struct TriangleBuilder {
    /// Material of the triangle.
    pub material: Material,

    /// Vertices of the triangle.
    pub vertices: [Point; 3],
}

impl TryFrom<TriangleBuilder> for Triangle {
    type Error = Error;

    fn try_from(builder: TriangleBuilder) -> Result<Self, Self::Error> {
        let TriangleBuilder { material, vertices } = builder;

        let v0 = vertices[0];
        let v1 = vertices[1];
        let v2 = vertices[2];

        let e0 = v1 - v0;
        let e1 = v2 - v0;
        let normal = e1
            .cross(e0)
            .normalize()
            .map_err(|_| Error::CollinearTriangleSides)?;

        let object_cache = ObjectCache::new(
            material,
            Default::default(),
            BoundingBox::from([v0, v1, v2]),
        );

        Ok(Self {
            object_cache,
            v0,
            v1,
            v2,
            e0,
            e1,
            normal,
        })
    }
}

impl Triangle {
    pub(crate) fn intersect<'a>(&self, object: &'a Shape, ray: &Ray) -> Vec<Intersection<'a>> {
        let dir_cross_e1 = ray.direction.cross(self.e1);
        let det = self.e0.dot(dir_cross_e1);

        if float::approx(det.abs(), 0.0) {
            return vec![];
        }

        let f = 1.0 / det;
        let p0_to_origin = ray.origin - self.v0;
        let u = f * p0_to_origin.dot(dir_cross_e1);

        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }

        let origin_cross_e0 = p0_to_origin.cross(self.e0);
        let v = f * ray.direction.dot(origin_cross_e0);

        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        vec![Intersection {
            t: f * self.e1.dot(origin_cross_e0),
            object,
            u: Some(u),
            v: Some(v),
        }]
    }

    pub(crate) fn normal_at(&self, _: Point) -> Vector {
        self.normal
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    #[test]
    fn constructing_a_triangle() {
        let v0 = Point::new(0.0, 1.0, 0.0);
        let v1 = Point::new(-1.0, 0.0, 0.0);
        let v2 = Point::new(1.0, 0.0, 0.0);

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [v0, v1, v2],
        })
        .unwrap();

        assert_eq!(triangle.v0, v0);
        assert_eq!(triangle.v1, v1);
        assert_eq!(triangle.v2, v2);
        assert_eq!(triangle.e0, Vector::new(-1.0, -1.0, 0.0));
        assert_eq!(triangle.e1, Vector::new(1.0, -1.0, 0.0));
        assert_eq!(triangle.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn trying_to_construct_a_triangle_with_collinear_sides() {
        let v0 = Point::new(1.0, 0.0, 0.0);
        let v1 = Point::new(2.0, 1.0, 0.0);
        let v2 = v0;

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [v0, v1, v2],
        });

        assert_eq!(triangle, Err(Error::CollinearTriangleSides));
    }

    #[test]
    fn finding_the_normal_on_a_triangle() {
        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let normal0 = triangle.normal_at(Point::new(0.0, 0.5, 0.0));
        let normal1 = triangle.normal_at(Point::new(-0.5, 0.75, 0.0));
        let normal2 = triangle.normal_at(Point::new(0.5, 0.25, 0.0));

        assert_eq!(normal0, triangle.normal);
        assert_eq!(normal1, triangle.normal);
        assert_eq!(normal2, triangle.normal);
    }

    #[test]
    fn intersecting_a_ray_parallel_to_the_triangle() {
        let object = Shape::Sphere(Default::default());

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let ray = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = triangle.intersect(&object, &ray);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p0_p2_edge() {
        let object = Shape::Sphere(Default::default());

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let ray = Ray {
            origin: Point::new(1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = triangle.intersect(&object, &ray);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p0_p1_edge() {
        let object = Shape::Sphere(Default::default());

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let ray = Ray {
            origin: Point::new(-1.0, 1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = triangle.intersect(&object, &ray);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_misses_the_p1_p2_edge() {
        let object = Shape::Sphere(Default::default());

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let ray = Ray {
            origin: Point::new(0.0, -1.0, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = triangle.intersect(&object, &ray);

        assert!(xs.is_empty())
    }

    #[test]
    fn a_ray_strikes_a_triangle() {
        let object = Shape::Sphere(Default::default());

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [
                Point::new(0.0, 1.0, 0.0),
                Point::new(-1.0, 0.0, 0.0),
                Point::new(1.0, 0.0, 0.0),
            ],
        })
        .unwrap();

        let ray = Ray {
            origin: Point::new(0.0, 0.5, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = triangle.intersect(&object, &ray);

        assert_eq!(xs.len(), 1);
        assert_approx!(xs[0].t, 2.0);
    }

    #[test]
    fn a_triangle_has_a_bounding_box() {
        let v0 = Point::new(-3.0, 7.0, 2.0);
        let v1 = Point::new(6.0, 2.0, -4.0);
        let v2 = Point::new(2.0, -1.0, -1.0);

        let triangle = Triangle::try_from(TriangleBuilder {
            material: Default::default(),
            vertices: [v0, v1, v2],
        })
        .unwrap();

        let bounding_box = triangle.object_cache.bounding_box;

        assert_eq!(bounding_box.min, Point::new(-3.0, -1.0, -4.0));
        assert_eq!(bounding_box.max, Point::new(6.0, 7.0, 2.0));
    }
}
