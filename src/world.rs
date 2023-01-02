use crate::{
    color::{self, Color},
    intersections::{Computation, Intersection},
    light::PointLight,
    ray::Ray,
    sphere::Sphere,
};

#[derive(Debug, Default)]
pub struct World {
    pub objects: Vec<Sphere>,
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
            acc + comps
                .intersection
                .object
                .material
                .lighting(light, comps.point, comps.eyev, comps.normalv)
                .unwrap_or(light.intensity)
        })
    }

    fn color_at(&self, ray: &Ray) -> Color {
        let xs = self.intersect(ray);

        Intersection::hit(xs).map_or(color::consts::BLACK, |hit| {
            self.shade_hit(hit.prepare_computations(ray))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        tuple::{Point, Vector},
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
    fn shade_hit_when_the_light_is_on_a_surface() {
        let light = PointLight {
            position: Point::new(0.0, 1.0, 0.0),
            intensity: color::consts::RED,
        };

        let s = Sphere::default();

        let w = World {
            objects: vec![s],
            lights: vec![light],
        };

        let r = Ray {
            origin: Point::new(0.0, 1.0, -1.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 1.0,
            object: &w.objects[0],
        };

        let comps = i.prepare_computations(&r);

        let c = w.shade_hit(comps);

        assert_eq!(c, w.lights[0].intensity);
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
        outer.material.ambient = 1.0;

        let inner = &mut w.objects[1];
        inner.material.ambient = 1.0;

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let c = w.color_at(&r);
        let inner = &w.objects[1];

        assert_eq!(c, inner.material.color);
    }
}
