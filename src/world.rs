use crate::{
    color::{self, Color},
    intersections::{Computation, Intersection},
    light::PointLight,
    ray::Ray,
    shape::Shape,
    tuple::Point,
};

#[derive(Debug, Default)]
pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<PointLight>,
}

impl World {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let mut xs: Vec<_> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();

        Intersection::sort(&mut xs);

        xs
    }

    fn shade_hit(&self, comps: Computation) -> Color {
        self.lights.iter().fold(color::consts::BLACK, |acc, light| {
            let in_shadow = self.is_shadowed(comps.over_point, light);

            acc + comps.intersection.object.as_ref().material.lighting(
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                in_shadow,
            )
        })
    }

    pub(crate) fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);

        Intersection::hit(xs).map_or(color::consts::BLACK, |hit| {
            self.shade_hit(hit.prepare_computations(ray))
        })
    }

    fn is_shadowed(&self, point: Point, light: &PointLight) -> bool {
        let point_to_light = light.position - point;

        let distance = point_to_light.magnitude();
        let point_to_light = if let Ok(vector) = point_to_light.normalize() {
            vector
        } else {
            return false;
        };

        let shadow_ray = Ray {
            origin: point,
            direction: point_to_light.normalize().unwrap(),
        };

        let xs = self.intersect(&shadow_ray);

        Intersection::hit(xs).map_or(false, |hit| hit.t < distance)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        shape::{Object, Sphere},
        transform::Transform,
        tuple::Vector,
        utils::test_world,
    };

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::default();

        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = test_world();
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
        let w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &w.objects[0],
        };

        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(
            c,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855,
            }
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            lights: vec![PointLight {
                position: Point::new(0.0, 0.25, 0.0),
                intensity: color::consts::WHITE,
            }],
            ..test_world()
        };

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 0.5,
            object: &w.objects[1],
        };

        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(
            c,
            Color {
                red: 0.90498,
                green: 0.90498,
                blue: 0.90498,
            }
        );
    }

    #[test]
    fn shade_hit_when_there_is_no_light() {
        let w = World {
            lights: Vec::new(),
            ..test_world()
        };

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 0.5,
            object: &w.objects[1],
        };

        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let c = w.color_at(&r);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = test_world();

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
                blue: 0.2855,
            }
        );
    }

    #[test]
    fn the_color_when_an_intersection_behind_the_ray() {
        let mut w = test_world();

        let outer = &mut w.objects[0];
        outer.as_mut().material.ambient = 1.0;

        let inner = &mut w.objects[1];
        inner.as_mut().material.ambient = 1.0;

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let c = w.color_at(&r);
        let inner = &w.objects[1];

        assert_eq!(c, inner.as_ref().material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = test_world();

        let p = Point::new(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = test_world();

        let p = Point::new(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = test_world();

        let p = Point::new(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = test_world();

        let p = Point::new(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn there_is_no_shadow_when_the_light_is_on_the_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        let light = PointLight {
            position: p,
            intensity: color::consts::WHITE,
        };

        let w = World {
            objects: Vec::new(),
            lights: vec![light],
        };

        assert!(!w.is_shadowed(p, &w.lights[0]));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Shape::Sphere(Sphere::default());
        let s2 = Shape::Sphere(Sphere(Object {
            transform: Transform::translation(0.0, 0.0, 10.0),
            ..Default::default()
        }));

        let light = PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let objects = vec![s1, s2.clone()];
        let lights = vec![light];

        let w = World { objects, lights };

        let r = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &s2,
        };

        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

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
