use std::cmp::Ordering;

use crate::{
    float,
    ray::Ray,
    shape::Shape,
    tuple::{Point, Vector},
};

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Shape,
}

#[derive(Debug)]
pub struct Computation<'a> {
    pub eyev: Vector,
    pub inside: bool,
    pub intersection: Intersection<'a>,
    pub n1: f64,
    pub n2: f64,
    pub normalv: Vector,
    pub over_point: Point,
    pub point: Point,
    pub reflectv: Vector,
    pub under_point: Point,
}

#[derive(Debug)]
pub struct IntersectionsCollection<'a> {
    pub intersections: Vec<Intersection<'a>>,
    containers: Vec<&'a Shape>,
}

#[macro_export]
macro_rules! intersections_vec {
    [$($i:expr),+] => {{
        IntersectionsCollection::from(vec![$($i),*])
    }};
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.t, other.t) && self.object == other.object
    }
}

impl<'a> From<Vec<Intersection<'a>>> for IntersectionsCollection<'a> {
    fn from(mut value: Vec<Intersection<'a>>) -> Self {
        value.sort_unstable_by(|i1, i2| {
            if float::approx(i1.t, i2.t) {
                Ordering::Equal
            } else if i1.t < i2.t {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        let visited = vec![];

        Self {
            intersections: value,
            containers: visited,
        }
    }
}

impl<'a> std::ops::Index<usize> for IntersectionsCollection<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.intersections[index]
    }
}

impl<'a> Computation<'a> {
    // https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    pub fn schlick(&self) -> f64 {
        let mut cos = self.eyev.dot(self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl<'a> IntersectionsCollection<'a> {
    pub fn prepare_computation(
        &mut self,
        world_ray: &Ray,
        intersection: Intersection<'a>,
    ) -> Computation<'a> {
        let point = world_ray.position(intersection.t);
        let eyev = -world_ray.direction;

        let normalv = intersection.object.normal_at(point);
        let inside = normalv.dot(eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };
        let reflectv = world_ray.direction.reflect(normalv);

        let over_point = point + normalv * float::EPSILON;
        let under_point = point - normalv * float::EPSILON;

        let hit = self.intersections.iter().find(|i| i.t > 0.0);

        let mut n1 = 1.0;
        let mut n2 = 1.0;

        for i in &self.intersections {
            if Some(i) == hit {
                if let Some(object) = self.containers.last() {
                    n1 = object.as_ref().material.index_of_refraction;
                }
            }

            if let Some(index) = self.containers.iter().position(|s| s == &i.object) {
                self.containers.remove(index);
            } else {
                self.containers.push(i.object);
            }

            if Some(i) == hit {
                if let Some(object) = self.containers.last() {
                    n2 = object.as_ref().material.index_of_refraction;
                }

                break;
            }
        }

        // Invalid internal usage (form other parts of the raytracer).
        #[allow(clippy::unwrap_used)]
        let index = self
            .intersections
            .iter()
            .position(|i| i == &intersection)
            .unwrap();
        self.intersections.remove(index);

        Computation {
            eyev,
            inside,
            intersection,
            n1,
            n2,
            normalv,
            over_point,
            point,
            reflectv,
            under_point,
        }
    }

    pub fn hit(&self) -> Option<Intersection<'a>> {
        Self::slice_hit(&self.intersections)
    }

    pub fn slice_hit(intersections: &[Intersection<'a>]) -> Option<Intersection<'a>> {
        intersections.iter().find(|i| i.t > 0.0).copied()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        float::EPSILON,
        material::{self, Material},
        ray::Ray,
        shape::{Object, Sphere},
        transform::Transform,
        tuple::{Point, Vector},
    };

    use super::*;

    fn glass_sphere() -> Shape {
        Shape::Sphere(Sphere(Object {
            material: Material {
                index_of_refraction: 1.5,
                transparency: 1.0,
                ..Default::default()
            },
            ..Default::default()
        }))
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Shape::Sphere(Default::default());

        let i = Intersection { t: 3.5, object: &s };

        assert_approx!(i.t, 3.5);
        assert_eq!(i.object, &s);
    }

    #[test]
    fn aggregating_intesrections() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };

        let xs = vec![&i1, &i2];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection { t: 1.0, object: &s };
        let i2 = Intersection { t: 2.0, object: &s };

        let xs = intersections_vec![i1, i2];

        assert_eq!(xs.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection {
            t: -1.0,
            object: &s,
        };
        let i2 = Intersection { t: 1.0, object: &s };

        let xs = intersections_vec![i1, i2];

        assert_eq!(xs.hit(), Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection {
            t: -2.0,
            object: &s,
        };
        let i2 = Intersection {
            t: -1.0,
            object: &s,
        };

        let xs = intersections_vec![i1, i2];

        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn sorting_a_vector_of_intersections() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };

        let xs = intersections_vec![i1, i2, i3, i4];

        assert_eq!(xs[0], i3);
        assert_eq!(xs[1], i4);
        assert_eq!(xs[2], i1);
        assert_eq!(xs[3], i2);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let s = Shape::Sphere(Default::default());

        let i1 = Intersection { t: 5.0, object: &s };
        let i2 = Intersection { t: 7.0, object: &s };
        let i3 = Intersection {
            t: -3.0,
            object: &s,
        };
        let i4 = Intersection { t: 2.0, object: &s };

        let xs = intersections_vec![i1, i2, i3, i4];

        assert_eq!(xs.hit(), Some(i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let s = Shape::Sphere(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection { t: 4.0, object: &s };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        assert_approx!(comps.intersection.t, 4.0);
        assert_eq!(comps.intersection.object, &s);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shape::Sphere(Default::default());

        let i = Intersection { t: 4.0, object: &s };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shape::Sphere(Default::default());

        let i = Intersection { t: 1.0, object: &s };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shape::Sphere(Sphere(Object {
            transform: Transform::translation(0.0, 0.0, 1.0),
            ..Default::default()
        }));

        let i = Intersection { t: 5.0, object: &s };

        let mut xs = intersections_vec!(i);
        let comps = xs.prepare_computation(&r, i);

        assert!(comps.over_point.0.z < -float::EPSILON / 2.0);
        assert!(comps.point.0.z > comps.over_point.0.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let s = Shape::Plane(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 1.0, -1.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &s,
        };

        let mut xs = intersections_vec!(i);
        let comps = xs.prepare_computation(&r, i);

        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Shape::Sphere(Sphere(Object {
            material: Material {
                index_of_refraction: 1.5,
                ..glass_sphere().as_ref().material.clone()
            },
            transform: Transform::try_scaling(2.0, 2.0, 2.0).unwrap(),
        }));

        let b = Shape::Sphere(Sphere(Object {
            material: Material {
                index_of_refraction: 2.0,
                ..glass_sphere().as_ref().material.clone()
            },
            transform: Transform::translation(0.0, 0.0, -0.25),
        }));

        let c = Shape::Sphere(Sphere(Object {
            material: Material {
                index_of_refraction: 2.5,
                ..glass_sphere().as_ref().material.clone()
            },
            transform: Transform::translation(0.0, 0.0, 0.25),
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, -4.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i0 = Intersection { t: 2.0, object: &a };
        let i1 = Intersection {
            t: 2.75,
            object: &b,
        };
        let i2 = Intersection {
            t: 3.25,
            object: &c,
        };
        let i3 = Intersection {
            t: 4.75,
            object: &b,
        };
        let i4 = Intersection {
            t: 5.25,
            object: &c,
        };
        let i5 = Intersection { t: 6.0, object: &a };

        let mut xs = intersections_vec![i0, i1, i2, i3, i4, i5];

        let comp0 = xs.prepare_computation(&r, i0);
        assert_approx!(comp0.n1, 1.0);
        assert_approx!(comp0.n2, 1.5);

        let comps1 = xs.prepare_computation(&r, i1);
        assert_approx!(comps1.n1, 1.5);
        assert_approx!(comps1.n2, 2.0);

        let comps2 = xs.prepare_computation(&r, i2);
        assert_approx!(comps2.n1, 2.0);
        assert_approx!(comps2.n2, 2.5);

        let comps3 = xs.prepare_computation(&r, i3);
        assert_approx!(comps3.n1, 2.5);
        assert_approx!(comps3.n2, 2.5);

        let comps4 = xs.prepare_computation(&r, i4);
        assert_approx!(comps4.n1, 2.5);
        assert_approx!(comps4.n2, 1.5);

        let comps5 = xs.prepare_computation(&r, i5);
        assert_approx!(comps5.n1, 1.5);
        assert_approx!(comps5.n2, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = Shape::Sphere(Sphere(Object {
            material: Material {
                index_of_refraction: material::consts::GLASS_INDEX_OF_REFRACTION,
                ..glass_sphere().as_ref().material.clone()
            },
            transform: Transform::translation(0.0, 0.0, 1.0),
        }));

        let i = Intersection { t: 5.0, object: &s };

        let mut xs = intersections_vec![i];

        let comps = xs.prepare_computation(&r, i);

        assert!(comps.under_point.0.z > EPSILON / 2.0);
        assert!(comps.point.0.z < comps.under_point.0.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let s = Shape::Sphere(Sphere(Object {
            material: glass_sphere().as_ref().material.clone(),
            ..Default::default()
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut xs = intersections_vec![
            Intersection {
                t: -2_f64.sqrt() / 2.0,
                object: &s,
            },
            Intersection {
                t: 2_f64.sqrt() / 2.0,
                object: &s,
            }
        ];

        let comps = xs.prepare_computation(&r, xs[1]);

        let reflectance = comps.schlick();

        assert_approx!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximatoin_with_a_perpendicular_viewing_angle() {
        let s = glass_sphere();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut xs = intersections_vec![
            Intersection {
                t: -1.0,
                object: &s
            },
            Intersection { t: 1.0, object: &s }
        ];

        let comps = xs.prepare_computation(&r, xs[1]);

        let reflectance = comps.schlick();

        assert_approx!(reflectance, 0.04);
    }

    #[test]
    fn the_schlick_approximation_with_small_andle_and_n2_greater_than_n1() {
        let s = glass_sphere();

        let r = Ray {
            origin: Point::new(0.0, 0.99, -2.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = intersections_vec![Intersection {
            t: 1.8589,
            object: &s
        }];

        let comps = xs.prepare_computation(&r, xs[0]);

        let reflectance = comps.schlick();

        assert_approx!(reflectance, 0.48873);
    }
}
