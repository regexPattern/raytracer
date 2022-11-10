use crate::color::{self, Color};
use crate::float;
use crate::intersection::Computation;
use crate::intersection::Intersection;
use crate::light::Light;
use crate::ray::Ray;
use crate::shape::Shapes;
use crate::tuple::Point;

pub const REFLECTION_LIMIT: u32 = 5;

pub struct World {
    pub objects: Vec<Shapes>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn color_at(&self, world_ray: &Ray, remaining: u32) -> Color {
        let xs = self.intersect(world_ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(&hit.comps(world_ray), remaining),
            None => color::BLACK,
        }
    }

    fn intersect(&self, world_ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<_> = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(world_ray))
            .collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit(&self, comps: &Computation, remaining: u32) -> Color {
        self.lights.iter().fold(color::BLACK, |shade, light| {
            let shadowed = self.is_shadowed(light, comps.over_point);
            let surface = shade
                + comps.i.object.shape().material.lighting(
                    &comps.i.object,
                    *light,
                    comps.over_point,
                    comps.eyev,
                    comps.normalv,
                    shadowed,
                );

            let reflected = self.reflected_color(comps, remaining);

            surface + reflected
        })
    }

    fn reflected_color(&self, comps: &Computation, remaining: u32) -> Color {
        let material = comps.i.object.shape().material;

        if remaining == 0 || float::approx(material.reflective, 0.0) {
            return color::BLACK;
        }

        let reflect_ray = Ray {
            origin: comps.over_point,
            direction: comps.reflectv,
        };

        let color = self.color_at(&reflect_ray, remaining - 1);

        color * material.reflective
    }

    fn is_shadowed(&self, light: &Light, world_point: Point) -> bool {
        let v = light.position - world_point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let ray = Ray {
            origin: world_point,
            direction,
        };

        let xs = self.intersect(&ray);

        if let Some(hit) = Intersection::hit(xs) {
            return hit.t < distance;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::material::{Material, Texture};
    use crate::matrix::Matrix;
    use crate::shape::{Plane, Shape, Sphere};
    use crate::tuple::Vector;
    use crate::{assert_approx, world};

    use super::*;

    fn test_default_world() -> World {
        let inner_sphere = Shapes::Sphere(Sphere(Shape {
            material: Material {
                diffuse: 0.7,
                specular: 0.2,
                texture: Texture::from(Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                }),
                ..Default::default()
            },
            ..Default::default()
        }));

        let outer_sphere = Shapes::Sphere(Sphere(Shape {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        }));

        let main_light = Light {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        World {
            objects: vec![inner_sphere, outer_sphere],
            lights: vec![main_light],
        }
    }

    #[test]
    fn the_default_world() {
        let light = Light {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        };

        let s1 = Shapes::Sphere(Sphere(Shape {
            material: Material {
                diffuse: 0.7,
                specular: 0.2,
                texture: Texture::from(Color {
                    red: 0.8,
                    green: 1.0,
                    blue: 0.6,
                }),
                ..Default::default()
            },
            ..Default::default()
        }));

        let s2 = Shapes::Sphere(Sphere(Shape {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        }));

        let world = test_default_world();

        assert!(world.lights.contains(&light));
        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = test_default_world();

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
        let world = test_default_world();

        let shape = world.objects[0];

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 4.0,
        };

        let comps = i.comps(&ray);

        let color = world.shade_hit(&comps, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        );
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let world = World {
            lights: vec![Light {
                position: Point::new(0.0, 0.25, 0.0),
                intensity: color::WHITE,
            }],
            ..test_default_world()
        };

        let shape = world.objects[1];

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection {
            object: shape,
            t: 0.5,
        };

        let comps = i.comps(&ray);

        let color = world.shade_hit(&comps, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.90498,
                green: 0.90498,
                blue: 0.90498
            }
        );
    }

    #[test]
    fn the_color_when_a_ray_misses() {
        let world = test_default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let color = world.color_at(&ray, world::REFLECTION_LIMIT);

        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = test_default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let color = world.color_at(&ray, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.38066,
                green: 0.47583,
                blue: 0.2855
            }
        )
    }

    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let mut world = test_default_world();

        let outer = &mut world.objects[0];
        if let Shapes::Sphere(shape) = outer {
            shape.0.material.ambient = 1.0;
        }

        let inner = &mut world.objects[1];
        if let Shapes::Sphere(shape) = inner {
            shape.0.material.ambient = 1.0;
        }

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let inner = &world.objects[1];

        let color = world.color_at(&ray, world::REFLECTION_LIMIT);

        assert_eq!(Texture::from(color), inner.shape().material.texture);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = test_default_world();
        let point = Point::new(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(&world.lights[0], point));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = test_default_world();
        let point = Point::new(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(&world.lights[0], point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = test_default_world();
        let point = Point::new(-20.0, 20.0, -20.0);

        assert!(!world.is_shadowed(&world.lights[0], point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = test_default_world();
        let point = Point::new(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(&world.lights[0], point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Shapes::Sphere(Sphere::default());

        let s2 = Shapes::Sphere(Sphere(Shape {
            transform: Matrix::translation(0.0, 0.0, 10.0),
            ..Default::default()
        }));

        let light = Light {
            position: Point::new(0.0, 0.0, -10.0),
            intensity: color::WHITE,
        };

        let objects = vec![s1, s2];
        let lights = vec![light];

        let world = World { objects, lights };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let i = Intersection { object: s2, t: 4.0 };

        let comps = i.comps(&ray);

        let color = world.shade_hit(&comps, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut world = test_default_world();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = &mut world.objects[1];
        shape.shape().material.ambient = 1.0;

        let shape = world.objects[1];

        let i = Intersection {
            object: shape,
            t: 1.0,
        };

        let comps = i.comps(&ray);

        let color = world.reflected_color(&comps, world::REFLECTION_LIMIT);

        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut world = test_default_world();

        let shape = Shapes::Plane(Plane(Shape {
            material: Material {
                reflective: 0.5,
                ..Default::default()
            },
            transform: Matrix::translation(0.0, -1.0, 0.0),
        }));

        world.objects.push(shape);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            object: shape,
            t: 2_f64.sqrt(),
        };

        let comps = i.comps(&ray);

        let color = world.reflected_color(&comps, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.19033,
                green: 0.23791,
                blue: 0.14274
            }
        );
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut world = test_default_world();

        let shape = Shapes::Plane(Plane(Shape {
            material: Material {
                reflective: 0.5,
                ..Default::default()
            },
            transform: Matrix::translation(0.0, -1.0, 0.0),
        }));

        world.objects.push(shape);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            object: shape,
            t: 2_f64.sqrt(),
        };

        let comps = i.comps(&ray);

        let color = world.shade_hit(&comps, world::REFLECTION_LIMIT);

        assert_eq!(
            color,
            Color {
                red: 0.87675,
                green: 0.92434,
                blue: 0.82918,
            }
        );
    }

    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut world = test_default_world();

        let shape = Shapes::Plane(Plane(Shape {
            material: Material {
                reflective: 0.5,
                ..Default::default()
            },
            transform: Matrix::translation(0.0, -1.0, 0.0),
        }));

        world.objects.push(shape);

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -3.0),
            direction: Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        };

        let i = Intersection {
            object: shape,
            t: 2_f64.sqrt(),
        };

        let comps = i.comps(&ray);

        let color = world.reflected_color(&comps, 0);

        assert_eq!(color, color::BLACK);
    }
}
