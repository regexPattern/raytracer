use crate::{
    color::{self, Color},
    float,
    intersections::{Computation, IntersectionsCollection},
    light::PointLight,
    ray::Ray,
    shape::Shape,
    tuple::Point,
};

pub(crate) const RECURSION_DEPTH: u8 = 5;

#[derive(Debug, Default)]
pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub(crate) fn color_at(&self, world_ray: &Ray, recursion_depth: u8) -> Color {
        let mut xs = self.intersect(world_ray);

        xs.hit().map_or(color::consts::BLACK, |hit| {
            self.shade_hit(xs.prepare_computation(world_ray, hit), recursion_depth)
        })
    }

    fn intersect(&self, world_ray: &Ray) -> IntersectionsCollection<'_> {
        self.objects
            .iter()
            .flat_map(|obj| obj.intersect(world_ray))
            .collect::<Vec<_>>()
            .into()
    }

    fn shade_hit(&self, comps: Computation, recursion_depth: u8) -> Color {
        self.lights.iter().fold(color::consts::BLACK, |acc, light| {
            let object = comps.intersection.object.as_ref();
            let in_shadow = self.is_shadowed(comps.over_point, light);

            let surface_color = object.material.lighting(
                object,
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                in_shadow,
            );

            let reflected_color = self.reflected_color(&comps, recursion_depth);
            let refracted_color = self.refracted_color(&comps, recursion_depth);

            let reflectance_color =
                if (object.material.reflectivity * object.material.transparency) > 0.0 {
                    let reflectance = comps.schlick();
                    reflected_color * reflectance + refracted_color * (1.0 - reflectance)
                } else {
                    reflected_color + refracted_color
                };

            acc + surface_color + reflectance_color
        })
    }

    fn is_shadowed(&self, world_point: Point, light: &PointLight) -> bool {
        let point_to_light = light.position - world_point;
        let distance = point_to_light.magnitude();

        let point_to_light = if let Ok(vector) = point_to_light.normalize() {
            vector
        } else {
            return false;
        };

        let shadow_ray = Ray {
            origin: world_point,
            direction: point_to_light,
        };

        let xs = self.intersect(&shadow_ray);
        let hit = IntersectionsCollection::slice_hit(&xs.intersections);

        hit.map_or(false, |hit| hit.t < distance)
    }

    fn reflected_color(&self, comps: &Computation<'_>, recursion_depth: u8) -> Color {
        let reflectiveness = comps.intersection.object.as_ref().material.reflectivity;

        if float::approx(reflectiveness, 0.0) || recursion_depth == 0 {
            return color::consts::BLACK;
        }

        let reflection_ray = Ray {
            origin: comps.over_point,
            direction: comps.reflectv,
        };

        self.color_at(&reflection_ray, recursion_depth - 1) * reflectiveness
    }

    fn refracted_color(&self, comps: &Computation<'_>, recursion_depth: u8) -> Color {
        let transparency = comps.intersection.object.as_ref().material.transparency;

        // Snell's Law: n1 * sin(oi) = n2 * sin(ot)
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = comps.eyev.dot(comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

        let is_total_internal_refraction = sin2_t > 1.0;

        if float::approx(transparency, 0.0) || recursion_depth == 0 || is_total_internal_refraction
        {
            return color::consts::BLACK;
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;

        let refraction_ray = Ray {
            origin: comps.under_point,
            direction,
        };

        self.color_at(&refraction_ray, recursion_depth - 1) * transparency
    }
}

#[cfg(test)]
pub mod test_utils {
    use crate::{
        material::Material,
        pattern::Pattern3D,
        shape::{Object, Sphere},
        transform::Transform,
    };

    use super::*;

    pub fn test_world() -> World {
        let light = PointLight {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let s1 = Shape::Sphere(Sphere(Object {
            material: Material {
                pattern: Pattern3D::Solid(color::Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                }),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        }));

        let s2 = Shape::Sphere(Sphere(Object {
            transform: Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
            ..Default::default()
        }));

        let objects = vec![s1, s2];
        let lights = vec![light];

        World { objects, lights }
    }

    #[test]
    fn the_default_test_world() {
        let light = PointLight {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::consts::WHITE,
        };

        let s1 = Shape::Sphere(Sphere(Object {
            material: Material {
                pattern: Pattern3D::Solid(Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                }),
                diffuse: 0.7,
                specular: 0.2,
                ..Default::default()
            },
            ..Default::default()
        }));

        let s2 = Shape::Sphere(Sphere(Object {
            transform: Transform::try_scaling(0.5, 0.5, 0.5).unwrap(),
            ..Default::default()
        }));

        let w = test_world();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        intersections::Intersection,
        intersections_vec,
        material::Material,
        pattern::Pattern3D,
        shape::{Object, Plane, Sphere},
        transform::Transform,
        tuple::Vector,
    };

    use super::{test_utils::test_world, *};

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

        assert_eq!(xs.intersections.len(), 4);
        assert_approx!(xs.intersections[0].t, 4.0);
        assert_approx!(xs.intersections[1].t, 4.5);
        assert_approx!(xs.intersections[2].t, 5.5);
        assert_approx!(xs.intersections[3].t, 6.0);
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

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

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

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

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
            lights: vec![],
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

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let c = w.color_at(&r, RECURSION_DEPTH);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let c = w.color_at(&r, RECURSION_DEPTH);

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

        let c = w.color_at(&r, RECURSION_DEPTH);
        let inner = &w.objects[1];

        assert_eq!(Pattern3D::Solid(c), inner.as_ref().material.pattern);
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
            objects: vec![],
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

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            c,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn the_reflected_color_for_a_non_reflective_material() {
        let mut w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let s = &mut w.objects[1];
        s.as_mut().material.ambient = 1.0;

        let i = Intersection {
            t: 1.0,
            object: &w.objects[1],
        };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.reflected_color(&comps, RECURSION_DEPTH);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let w = test_world();

        let s = Shape::Plane(Plane(Object {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &s,
        };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.reflected_color(&comps, RECURSION_DEPTH);

        assert_eq!(
            c,
            Color {
                red: 0.19033,
                green: 0.23791,
                blue: 0.14275,
            }
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let w = test_world();

        let s = Shape::Plane(Plane(Object {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let r = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &s,
        };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            c,
            Color {
                red: 0.87676,
                green: 0.92435,
                blue: 0.82918,
            }
        );
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let lower = Shape::Plane(Plane(Object {
            transform: Transform::translation(0.0, -1.0, 0.0),
            material: Material {
                reflectivity: 1.0,
                ..Default::default()
            },
        }));

        let upper = Shape::Plane(Plane(Object {
            transform: Transform::translation(0.0, 1.0, 0.0),
            material: lower.as_ref().material.clone(),
        }));

        let light = PointLight {
            position: Point::new(0.0, 0.0, 0.0),
            intensity: color::consts::WHITE,
        };

        let objects = vec![lower, upper];
        let lights = vec![light];

        let w = World { objects, lights };

        let r = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        // This should not stack overflow, so it should not panic.
        w.color_at(&r, RECURSION_DEPTH);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let s = Shape::Plane(Plane(Object {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let mut w = test_world();
        w.objects.push(s);

        let r = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &w.objects[2],
        };

        let mut xs = intersections_vec![i];
        let comps = xs.prepare_computation(&r, i);

        let c = w.reflected_color(&comps, 0);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = intersections_vec![
            Intersection {
                t: 4.0,
                object: &w.objects[0],
            },
            Intersection {
                t: 6.0,
                object: &w.objects[0],
            }
        ];

        let comps = xs.prepare_computation(&r, xs[0]);

        let c = w.refracted_color(&comps, RECURSION_DEPTH);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut w = test_world();

        let s = &mut w.objects[0];
        s.as_mut().material.transparency = 1.0;
        s.as_mut().material.index_of_refraction = 1.5;

        let r = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let mut xs = intersections_vec![
            Intersection {
                t: 4.0,
                object: &w.objects[0],
            },
            Intersection {
                t: 6.0,
                object: &w.objects[0],
            }
        ];

        let comps = xs.prepare_computation(&r, xs[0]);

        let c = w.refracted_color(&comps, 0);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut w = test_world();

        let s = &mut w.objects[0];
        s.as_mut().material.transparency = 1.0;
        s.as_mut().material.index_of_refraction = 1.5;

        let r = Ray {
            origin: Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let mut xs = intersections_vec![
            Intersection {
                t: -2_f64.sqrt() / 2.0,
                object: &w.objects[0],
            },
            Intersection {
                t: 2_f64.sqrt() / 2.0,
                object: &w.objects[0],
            }
        ];

        let comps = xs.prepare_computation(&r, xs[1]);

        let c = w.refracted_color(&comps, RECURSION_DEPTH);

        assert_eq!(c, color::consts::BLACK);
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = test_world();

        let floor = Shape::Plane(Plane(Object {
            material: Material {
                index_of_refraction: 1.5,
                transparency: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ball = Shape::Sphere(Sphere(Object {
            material: Material {
                ambient: 0.5,
                pattern: Pattern3D::Solid(color::consts::RED),
                ..Default::default()
            },
            transform: Transform::translation(0.0, -3.5, -0.5),
        }));

        w.objects.push(floor);
        w.objects.push(ball);

        let r = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let mut xs = intersections_vec![Intersection {
            t: 2_f64.sqrt(),
            object: &w.objects[2],
        }];

        let comps = xs.prepare_computation(&r, xs[0]);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            c,
            Color {
                red: 0.93642,
                green: 0.68642,
                blue: 0.68642
            }
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_and_transparent_material() {
        let mut w = test_world();

        let r = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let floor = Shape::Plane(Plane(Object {
            material: Material {
                index_of_refraction: 1.5,
                reflectivity: 0.5,
                transparency: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ball = Shape::Sphere(Sphere(Object {
            material: Material {
                ambient: 0.5,
                pattern: Pattern3D::Solid(color::consts::RED),
                ..Default::default()
            },
            transform: Transform::translation(0.0, -3.5, -0.5),
        }));

        w.objects.push(floor);
        w.objects.push(ball);

        let mut xs = intersections_vec![Intersection {
            t: 2_f64.sqrt(),
            object: &w.objects[2]
        }];

        let comps = xs.prepare_computation(&r, xs[0]);

        let c = w.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            c,
            Color {
                red: 0.93391,
                green: 0.69643,
                blue: 0.69243
            }
        );
    }
}
