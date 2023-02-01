use crate::{
    color::{self, Color},
    float,
    intersection::{Computation, Intersection},
    light::Light,
    ray::Ray,
    shape::Shape,
    tuple::Point,
};

pub(crate) const RECURSION_DEPTH: u8 = 5;

#[derive(Debug, Default)]
pub struct World {
    pub objects: Vec<Shape>,
    pub lights: Vec<Light>,
}

impl World {
    pub(crate) fn color_at(&self, ray: &Ray, recursion_depth: u8) -> Color {
        let mut xs = self.intersect(ray);

        Intersection::hit(&mut xs).map_or(color::consts::BLACK, |hit| {
            self.shade_hit(hit.prepare_computation(ray, xs), recursion_depth)
        })
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection<'_>> {
        let mut intersections: Vec<_> = self
            .objects
            .iter()
            .flat_map(|obj| obj.intersect(ray))
            .collect();

        Intersection::sort(&mut intersections);
        intersections
    }

    fn shade_hit(&self, comps: Computation, recursion_depth: u8) -> Color {
        self.lights.iter().fold(color::consts::BLACK, |acc, light| {
            let object = comps.intersection.object;
            let material = &object.as_ref().material;

            let light_intensity = light.intensity_at(self, comps.over_point);

            let surface_color = material.lighting(
                object,
                light,
                comps.over_point,
                comps.eyev,
                comps.normalv,
                light_intensity,
            );

            let reflected_color = self.reflected_color(&comps, recursion_depth);
            let refracted_color = self.refracted_color(&comps, recursion_depth);

            let reflectance_color = if (material.reflectivity * material.transparency) > 0.0 {
                let reflectance = comps.schlick();
                reflected_color * reflectance + refracted_color * (1.0 - reflectance)
            } else {
                reflected_color + refracted_color
            };

            acc + surface_color + reflectance_color
        })
    }

    pub(crate) fn is_shadowed(&self, light_position: Point, point: Point) -> bool {
        let point_to_light = light_position - point;
        let distance = point_to_light.magnitude();

        let point_to_light = if let Ok(vector) = point_to_light.normalize() {
            vector
        } else {
            return false;
        };

        let shadow_ray = Ray {
            origin: point,
            direction: point_to_light,
        };

        let mut xs = self.intersect(&shadow_ray);
        let hit = Intersection::hit(&mut xs);

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
// This base world is used in other modules for testing purposes.
pub(crate) fn test_world() -> World {
    use crate::{
        light::PointLight,
        material::Material,
        pattern::Pattern3D,
        shape::{ShapeBuilder, Sphere},
        transform::Transform,
    };

    let light = Light::Point(PointLight {
        position: Point::new(-10.0, 10.0, -10.0),
        intensity: color::consts::WHITE,
    });

    let object0 = Shape::Sphere(Sphere::from(ShapeBuilder {
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

    let object1 = Shape::Sphere(Sphere::from(ShapeBuilder {
        transform: Transform::scaling(0.5, 0.5, 0.5).unwrap(),
        ..Default::default()
    }));

    World {
        objects: vec![object0, object1],
        lights: vec![light],
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        assert_approx,
        intersection::Intersection,
        light::PointLight,
        material::Material,
        pattern::Pattern3D,
        shape::{Plane, ShapeBuilder, Sphere},
        transform::Transform,
        tuple::Vector,
    };

    use super::{test_world, *};

    #[test]
    fn creating_a_world() {
        let world = World::default();

        assert_eq!(world.objects.len(), 0);
        assert_eq!(world.lights.len(), 0);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = test_world();
        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = world.intersect(&ray);

        assert_eq!(xs.len(), 4);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 4.5);
        assert_approx!(xs[2].t, 5.5);
        assert_approx!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &world.objects[0],
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855,
            }
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let world = World {
            lights: vec![Light::Point(PointLight {
                position: Point::new(0.0, 0.25, 0.0),
                intensity: color::consts::WHITE,
            })],
            ..test_world()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 0.5,
            object: &world.objects[1],
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.90498,
                green: 0.90498,
                blue: 0.90498,
            }
        );
    }

    #[test]
    fn shade_hit_when_there_is_no_light() {
        let world = World {
            lights: vec![],
            ..test_world()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 0.5,
            object: &world.objects[1],
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let color_at = world.color_at(&ray, RECURSION_DEPTH);

        assert_eq!(color_at, color::consts::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let color_at = world.color_at(&ray, RECURSION_DEPTH);

        assert_eq!(
            color_at,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855,
            }
        );
    }

    #[test]
    fn the_color_when_an_intersection_behind_the_ray() {
        let mut world = test_world();

        let outer_object = &mut world.objects[0];
        outer_object.as_mut().material = Material {
            ambient: 1.0,
            ..outer_object.as_ref().material.clone()
        };

        let inner_object = &mut world.objects[1];
        inner_object.as_mut().material = Material {
            ambient: 1.0,
            ..inner_object.as_ref().material.clone()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let color_at = world.color_at(&ray, RECURSION_DEPTH);
        let inner = &world.objects[1];

        assert_eq!(Pattern3D::Solid(color_at), inner.as_ref().material.pattern);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = test_world();

        let point = Point::new(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(Point::new(-10.0, 10.0, -10.0), point));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = test_world();

        let point = Point::new(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(Point::new(-10.0, 10.0, -10.0), point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = test_world();

        let point = Point::new(-20.0, 20.0, -20.0);

        assert!(!world.is_shadowed(Point::new(-10.0, 10.0, -10.0), point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = test_world();

        let point = Point::new(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(Point::new(-10.0, 10.0, -10.0), point));
    }

    #[test]
    fn there_is_no_shadow_when_the_light_is_on_the_point() {
        let point = Point::new(1.0, 2.0, 3.0);

        let light = Light::Point(PointLight {
            position: point,
            intensity: color::consts::WHITE,
        });

        let world = World {
            objects: vec![],
            lights: vec![light],
        };

        assert!(!world.is_shadowed(Point::new(-10.0, 10.0, -10.0), point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let object0 = Shape::Sphere(Default::default());

        let object1 = Shape::Sphere(Sphere::from(ShapeBuilder {
            transform: Transform::translation(0.0, 0.0, 10.0),
            ..Default::default()
        }));

        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::consts::WHITE,
        });

        let world = World {
            objects: vec![object0, object1.clone()],
            lights: vec![light],
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            t: 4.0,
            object: &object1,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn the_reflected_color_for_a_non_reflective_material() {
        let mut world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let object = &mut world.objects[1];
        object.as_mut().material = Material {
            ambient: 1.0,
            ..object.as_ref().material.clone()
        };

        let i = Intersection {
            t: 1.0,
            object: &world.objects[1],
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.reflected_color(&comps, RECURSION_DEPTH);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let world = test_world();

        let object = Shape::Plane(Plane::from(ShapeBuilder {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &object,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.reflected_color(&comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.19033,
                green: 0.23791,
                blue: 0.14275,
            }
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let world = test_world();

        let object = Shape::Plane(Plane::from(ShapeBuilder {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &object,
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.87676,
                green: 0.92435,
                blue: 0.82918,
            }
        );
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let lower_object = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                reflectivity: 1.0,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let upper_object = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: lower_object.as_ref().material.clone(),
            transform: Transform::translation(0.0, 1.0, 0.0),
        }));

        let light = Light::Point(PointLight {
            position: Point::new(0.0, 0.0, 0.0),
            intensity: color::consts::WHITE,
        });

        let world = World {
            objects: vec![lower_object, upper_object],
            lights: vec![light],
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        // This should not stack overflow, so it should not panic.
        world.color_at(&ray, RECURSION_DEPTH);
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let object = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                reflectivity: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let mut w = test_world();
        w.objects.push(object);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            t: 2_f64.sqrt(),
            object: &w.objects[2],
            u: None,
            v: None,
        };

        let comps = i.prepare_computation(&ray, [i]);

        let shade = w.reflected_color(&comps, 0);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = [
            Intersection {
                t: 4.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
            Intersection {
                t: 6.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
        ];

        let comps = xs[0].prepare_computation(&ray, xs);

        let shade = world.refracted_color(&comps, RECURSION_DEPTH);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let mut world = test_world();

        let object = &mut world.objects[0];
        object.as_mut().material = Material {
            index_of_refraction: 1.5,
            transparency: 1.0,
            ..object.as_ref().material.clone()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = [
            Intersection {
                t: 4.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
            Intersection {
                t: 6.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
        ];

        let comps = xs[0].prepare_computation(&ray, xs);

        let shade = world.refracted_color(&comps, 0);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let mut world = test_world();

        let object = &mut world.objects[0];
        object.as_mut().material = Material {
            index_of_refraction: 1.5,
            transparency: 1.0,
            ..object.as_ref().material.clone()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let xs = [
            Intersection {
                t: -2_f64.sqrt() / 2.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
            Intersection {
                t: 2_f64.sqrt() / 2.0,
                object: &world.objects[0],
                u: None,
                v: None,
            },
        ];

        let comps = xs[1].prepare_computation(&ray, xs);

        let shade = world.refracted_color(&comps, RECURSION_DEPTH);

        assert_eq!(shade, color::consts::BLACK);
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut world = test_world();

        let floor = Shape::Plane(Plane::from(ShapeBuilder {
            material: Material {
                index_of_refraction: 1.5,
                transparency: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ball = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                ambient: 0.5,
                pattern: Pattern3D::Solid(color::consts::RED),
                ..Default::default()
            },
            transform: Transform::translation(0.0, -3.5, -0.5),
        }));

        world.objects.push(floor);
        world.objects.push(ball);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let xs = [Intersection {
            t: 2_f64.sqrt(),
            object: &world.objects[2],
            u: None,
            v: None,
        }];

        let comps = xs[0].prepare_computation(&ray, xs);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.93642,
                green: 0.68642,
                blue: 0.68642
            }
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_and_transparent_material() {
        let mut world = test_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let floor = Shape::Plane(Plane::from(ShapeBuilder {
            material: Material {
                index_of_refraction: 1.5,
                reflectivity: 0.5,
                transparency: 0.5,
                ..Default::default()
            },
            transform: Transform::translation(0.0, -1.0, 0.0),
        }));

        let ball = Shape::Sphere(Sphere::from(ShapeBuilder {
            material: Material {
                ambient: 0.5,
                pattern: Pattern3D::Solid(color::consts::RED),
                ..Default::default()
            },
            transform: Transform::translation(0.0, -3.5, -0.5),
        }));

        world.objects.push(floor);
        world.objects.push(ball);

        let xs = [Intersection {
            t: 2_f64.sqrt(),
            object: &world.objects[2],
            u: None,
            v: None,
        }];

        let comps = xs[0].prepare_computation(&ray, xs);

        let shade = world.shade_hit(comps, RECURSION_DEPTH);

        assert_eq!(
            shade,
            Color {
                red: 0.93391,
                green: 0.69643,
                blue: 0.69243
            }
        );
    }

    #[test]
    fn is_shadowed_test_for_occlusion_between_two_points() {
        let world = test_world();
        let light_position = Point::new(-10.0, -10.0, -10.0);

        assert!(!world.is_shadowed(light_position, Point::new(-10.0, -10.0, 10.0)));
        assert!(world.is_shadowed(light_position, Point::new(10.0, 10.0, 10.0)));
        assert!(!world.is_shadowed(light_position, Point::new(-20.0, -20.0, -20.0)));
        assert!(!world.is_shadowed(light_position, Point::new(-5.0, -5.0, -5.0)));
    }
}
