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
    pub u: Option<f64>,
    pub v: Option<f64>,
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

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Self) -> bool {
        float::approx(self.t, other.t)
            && self.object == other.object
            && float::approx_some(self.u, other.u)
            && float::approx_some(self.v, other.v)
    }
}

impl<'a> Intersection<'a> {
    pub fn prepare_computation<T>(self, ray: &Ray, intersections: T) -> Computation<'a>
    where
        T: IntoIterator<Item = Intersection<'a>>,
    {
        let point = ray.position(self.t);
        let eyev = -ray.direction;

        let normalv = self.object.normal_at(point, &self);
        let inside = normalv.dot(eyev) < 0.0;
        let normalv = if inside { -normalv } else { normalv };
        let reflectv = ray.direction.reflect(normalv);

        let over_point = point + normalv * float::EPSILON;
        let under_point = point - normalv * float::EPSILON;

        let (n1, n2) = self.find_n1_and_n2(intersections);

        Computation {
            eyev,
            inside,
            intersection: self,
            n1,
            n2,
            normalv,
            over_point,
            point,
            reflectv,
            under_point,
        }
    }

    fn find_n1_and_n2<T>(&self, intersections: T) -> (f64, f64)
    where
        T: IntoIterator<Item = Intersection<'a>>,
    {
        let (mut n1, mut n2) = (1.0, 1.0);
        let mut visited: Vec<&Shape> = vec![];

        let hit = Some(self);

        for i in intersections {
            if Some(&i) == hit {
                if let Some(object) = visited.last() {
                    n1 = object.as_ref().material.index_of_refraction;
                }
            }

            if let Some(index) = visited.iter().position(|s| s == &i.object) {
                visited.remove(index);
            } else {
                visited.push(i.object);
            }

            if Some(&i) == hit {
                if let Some(object) = visited.last() {
                    n2 = object.as_ref().material.index_of_refraction;
                }

                break;
            }
        }

        (n1, n2)
    }

    pub fn sort(intersections: &mut [Intersection<'_>]) {
        intersections.sort_unstable_by(|i1, i2| {
            if float::approx(i1.t, i2.t) {
                std::cmp::Ordering::Equal
            } else if i1.t < i2.t {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
    }

    pub fn hit(intersections: &mut [Intersection<'a>]) -> Option<Intersection<'a>> {
        Self::sort(intersections);
        intersections.iter().find(|i| i.t > 0.0).copied()
    }
}

impl<'a> Computation<'a> {
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

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        material::Material,
        shape::{ShapeBuilder, Sphere},
        transform::Transform,
    };

    use super::*;

    fn glass_sphere() -> Shape {
        Shape::Sphere(Sphere::from(ShapeBuilder {
            material: glass_material(),
            ..Default::default()
        }))
    }

    fn glass_material() -> Material {
        Material {
            index_of_refraction: 1.5,
            transparency: 1.0,
            ..Default::default()
        }
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let o = glass_sphere();

        let i = Intersection {
            t: 3.5,
            object: &o,
            u: None,
            v: None,
        };

        assert_approx!(i.t, 3.5);
        assert_eq!(i.object, &o);
    }

    #[test]
    fn aggregating_intersections() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: 1.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 2.0,
            object: &o,
            u: None,
            v: None,
        };

        let xs = vec![&i0, &i1];

        assert_eq!(xs.len(), 2);
        assert_approx!(xs[0].t, 1.0);
        assert_approx!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: 1.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 2.0,
            object: &o,
            u: None,
            v: None,
        };

        let mut xs = [i0, i1];

        assert_eq!(Intersection::hit(&mut xs), Some(i0));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: -1.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 1.0,
            object: &o,
            u: None,
            v: None,
        };

        let mut xs = [i0, i1];

        assert_eq!(Intersection::hit(&mut xs), Some(i1));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: -2.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: -1.0,
            object: &o,
            u: None,
            v: None,
        };

        let mut xs = [i0, i1];

        assert_eq!(Intersection::hit(&mut xs), None);
    }

    #[test]
    fn sorting_a_vector_of_intersections() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: 5.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 7.0,
            object: &o,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: -3.0,
            object: &o,
            u: None,
            v: None,
        };
        let i3 = Intersection {
            t: 2.0,
            object: &o,
            u: None,
            v: None,
        };

        let mut xs = [i0, i1, i2, i3];

        Intersection::sort(&mut xs);

        assert_eq!(xs[0], i2);
        assert_eq!(xs[1], i3);
        assert_eq!(xs[2], i0);
        assert_eq!(xs[3], i1);
    }

    #[test]
    fn the_hit_is_always_the_lowest_non_negative_intersection() {
        let o = glass_sphere();

        let i0 = Intersection {
            t: 5.0,
            object: &o,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 7.0,
            object: &o,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: -3.0,
            object: &o,
            u: None,
            v: None,
        };
        let i3 = Intersection {
            t: 2.0,
            object: &o,
            u: None,
            v: None,
        };

        let mut xs = [i0, i1, i2, i3];

        assert_eq!(Intersection::hit(&mut xs), Some(i3));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let o = glass_sphere();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert_approx!(comps.intersection.t, 4.0);
        assert_eq!(comps.intersection.object, &o);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let o = glass_sphere();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert!(!comps.inside);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let o = glass_sphere();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 1.0,
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let o = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(0.0, 0.0, 1.0),
            ..Default::default()
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 5.0,
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert!(comps.over_point.0.z < -float::EPSILON / 2.0);
        assert!(comps.point.0.z > comps.over_point.0.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let o = Shape::Plane(Default::default());

        let r = Ray {
            origin: Point::new(0.0, 1.0, -1.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let a = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                index_of_refraction: 1.5,
                ..glass_material()
            },
            transform: Transform::scaling(2.0, 2.0, 2.0).unwrap(),
        }));

        let b = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                index_of_refraction: 2.0,
                ..glass_material()
            },
            transform: Transform::translation(0.0, 0.0, -0.25),
        }));

        let c = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                index_of_refraction: 2.5,
                ..glass_material()
            },
            transform: Transform::translation(0.0, 0.0, 0.25),
        }));

        let i0 = Intersection {
            t: 2.0,
            object: &a,
            u: None,
            v: None,
        };
        let i1 = Intersection {
            t: 2.75,
            object: &b,
            u: None,
            v: None,
        };
        let i2 = Intersection {
            t: 3.25,
            object: &c,
            u: None,
            v: None,
        };
        let i3 = Intersection {
            t: 4.75,
            object: &b,
            u: None,
            v: None,
        };
        let i4 = Intersection {
            t: 5.25,
            object: &c,
            u: None,
            v: None,
        };
        let i5 = Intersection {
            t: 6.0,
            object: &a,
            u: None,
            v: None,
        };

        let xs = [i0, i1, i2, i3, i4, i5];

        let (n1, n2) = i0.find_n1_and_n2(xs);
        assert_approx!(n1, 1.0);
        assert_approx!(n2, 1.5);

        let (n1, n2) = i1.find_n1_and_n2(xs);
        assert_approx!(n1, 1.5);
        assert_approx!(n2, 2.0);

        let (n1, n2) = i2.find_n1_and_n2(xs);
        assert_approx!(n1, 2.0);
        assert_approx!(n2, 2.5);

        let (n1, n2) = i3.find_n1_and_n2(xs);
        assert_approx!(n1, 2.5);
        assert_approx!(n2, 2.5);

        let (n1, n2) = i4.find_n1_and_n2(xs);
        assert_approx!(n1, 2.5);
        assert_approx!(n2, 1.5);

        let (n1, n2) = i5.find_n1_and_n2(xs);
        assert_approx!(n1, 1.5);
        assert_approx!(n2, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let o = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: glass_material(),
            transform: Transform::translation(0.0, 0.0, 1.0),
        }));

        let i = Intersection {
            t: 5.0,
            object: &o,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&r, [i]);

        assert!(comps.under_point.0.z > float::EPSILON / 2.0);
        assert!(comps.point.0.z < comps.under_point.0.z);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let o = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: glass_material(),
            ..Default::default()
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = [
            Intersection {
                t: -2_f64.sqrt() / 2.0,
                object: &o,
                u: None,
                v: None,
            },
            Intersection {
                t: 2_f64.sqrt() / 2.0,
                object: &o,
                u: None,
                v: None,
            },
        ];

        let comps = xs[1].prepare_computation(&r, xs);

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

        let xs = [
            Intersection {
                t: -1.0,
                object: &s,
                u: None,
                v: None,
            },
            Intersection {
                t: 1.0,
                object: &s,
                u: None,
                v: None,
            },
        ];

        let comps = xs[1].prepare_computation(&r, xs);

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

        let xs = [Intersection {
            t: 1.8589,
            object: &s,
            u: None,
            v: None,
        }];

        let comps = xs[0].prepare_computation(&r, xs);

        let reflectance = comps.schlick();

        assert_approx!(reflectance, 0.48873);
    }
}
