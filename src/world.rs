use crate::color::{self, Color};
use crate::intersection::Intersection;
use crate::intersection::PreparedIntersection;
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::sphere::Sphere;
use crate::shape::{Figure, Shapes};
use crate::tuple::Point;

pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Shapes>,
}

impl Default for World {
    fn default() -> Self {
        let objects = vec![
            Shapes::Sphere(Sphere(Figure {
                material: Material {
                    color: Color {
                        red: 0.8,
                        green: 1.0,
                        blue: 0.6,
                    },
                    diffuse: 0.7,
                    specular: 0.2,
                    ..Default::default()
                },
                ..Default::default()
            })),
            Shapes::Sphere(Sphere(Figure {
                transform: Matrix::scaling(0.5, 0.5, 0.5),
                ..Default::default()
            })),
        ];

        let lights = vec![PointLight {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        }];

        Self { objects, lights }
    }
}

impl World {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<_> = self.objects.iter().flat_map(|o| o.intersect(ray)).collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit(&self, comps: &PreparedIntersection) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        self.lights.iter().fold(color::BLACK, |shade, light| {
            shade
                + comps.object.shape().material.lighting(
                    light,
                    comps.over_point,
                    comps.eyev,
                    comps.normalv,
                    shadowed,
                )
        })
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(&hit.prepare(ray)),
            None => color::BLACK,
        }
    }

    fn is_shadowed(&self, point: Point) -> bool {
        let v = self.lights[0].position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray {
            origin: point,
            direction,
        };

        let xs = self.intersect(&r);

        if let Some(hit) = Intersection::hit(xs) {
            return hit.t < distance;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;
    use crate::tuple::Vector;

    use super::*;

    #[test]
    fn the_default_world() {
        let light = PointLight {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let s1 = Shapes::Sphere(Sphere(Figure {
            material: Material {
                color: Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                },
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        }));

        let s2 = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        }));

        let w = World::default();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = w.intersect(&r);

        assert_eq!(xs.len(), 4);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 4.5);
        assert_approx!(xs[2].t, 5.5);
        assert_approx!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = w.objects[0];
        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = i.prepare(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(
            c,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![PointLight {
                position: Point::new(0.0, 0.25, 0.0),
                intensity: color::WHITE,
            }],
            ..World::default()
        };
        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = w.objects[1];
        let i = Intersection {
            t: 0.5,
            object: shape,
        };

        let comps = i.prepare(&r);
        let c = w.shade_hit(&comps);

        assert_eq!(
            c,
            Color {
                red: 0.90498,
                green: 0.90498,
                blue: 0.90498
            }
        );
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = World::default();
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let c = w.color_at(&r);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let c = w.color_at(&r);

        assert_eq!(
            c,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        )
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut w = World::default();

        let outer = &mut w.objects[0];
        if let Shapes::Sphere(s) = outer {
            s.0.material.ambient = 1.0;
        }

        let inner = &mut w.objects[1];
        if let Shapes::Sphere(s) = inner {
            s.0.material.ambient = 1.0;
        }

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let inner = &w.objects[1];

        let c = w.color_at(&r);

        assert_eq!(c, inner.shape().material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::default();
        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Shapes::Sphere(Sphere::default());

        let s2 = Shapes::Sphere(Sphere(Figure {
            transform: Matrix::translation(0.0, 0.0, 10.0),
            ..Default::default()
        }));

        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let objects = vec![s1, s2];
        let lights = vec![light];

        let w = World { objects, lights };

        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection { t: 4.0, object: s2 };

        let comps = i.prepare(&r);

        let c = w.shade_hit(&comps);

        assert_eq!(
            c,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }
}
