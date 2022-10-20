use crate::color::{self, Color};
use crate::intersection::Intersection;
use crate::intersection::MetaData;
use crate::light::PointLight;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shape::{Shape, Shapes};
use crate::tuple::Point;

pub struct World {
    pub lights: Vec<PointLight>,
    pub objects: Vec<Shapes>,
}

impl Default for World {
    fn default() -> Self {
        let objects = vec![
            Shapes::Sphere(Shape {
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
            }),
            Shapes::Sphere(Shape {
                transform: Matrix::scaling(0.5, 0.5, 0.5),
                ..Default::default()
            }),
        ];

        let lights = vec![PointLight {
            position: Point::new(-10.0, 10.0, -10.0),
            intensity: color::WHITE,
        }];

        Self { objects, lights }
    }
}

impl World {
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut xs: Vec<_> = self.objects.iter().flat_map(|o| o.intersect(ray)).collect();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    fn shade_hit(&self, comps: MetaData) -> Color {
        let shadowed = self.is_shadowed(comps.over_point);
        self.lights.iter().fold(color::BLACK, |shade, light| {
            shade
                + comps.i.object.shape().material.lighting(
                    *light,
                    comps.over_point,
                    comps.eyev,
                    comps.normalv,
                    shadowed,
                )
        })
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let xs = self.intersect(ray);
        match Intersection::hit(xs) {
            Some(hit) => self.shade_hit(hit.comps(ray)),
            None => color::BLACK,
        }
    }

    fn is_shadowed(&self, point: Point) -> bool {
        self.lights.iter().any(|light| {
            let direction = light.position - point;
            let distance = direction.magnitude();
            let direction = direction.normalize();

            let ray = Ray {
                origin: point,
                direction,
            };

            let xs = self.intersect(ray);

            if let Some(hit) = Intersection::hit(xs) {
                return hit.t < distance;
            }

            false
        })
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

        let s1 = Shapes::Sphere(Shape {
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
        });

        let s2 = Shapes::Sphere(Shape {
            transform: Matrix::scaling(0.5, 0.5, 0.5),
            ..Default::default()
        });

        let world = World::default();

        assert!(world.lights.contains(&light));
        assert!(world.objects.contains(&s1));
        assert!(world.objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let world = World::default();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let xs = world.intersect(ray);

        assert_eq!(xs.len(), 4);
        assert_approx!(xs[0].t, 4.0);
        assert_approx!(xs[1].t, 4.5);
        assert_approx!(xs[2].t, 5.5);
        assert_approx!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = world.objects[0];

        let i = Intersection {
            t: 4.0,
            object: shape,
        };

        let comps = i.comps(ray);
        let c = world.shade_hit(comps);

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
        let world = World {
            lights: vec![PointLight {
                position: Point::new(0.0, 0.25, 0.0),
                intensity: color::WHITE,
            }],
            ..Default::default()
        };

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let shape = world.objects[1];

        let i = Intersection {
            t: 0.5,
            object: shape,
        };

        let comps = i.comps(ray);
        let c = world.shade_hit(comps);

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
        let world = World::default();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 1.0, 0.0),
        };

        let color = world.color_at(ray);

        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn the_color_when_a_ray_hits() {
        let world = World::default();

        let ray = Ray {
            origin: Point::new(0.0, 0.0, -5.0),
            direction: Vector::new(0.0, 0.0, 1.0),
        };

        let color = world.color_at(ray);

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
        let mut world = World::default();

        let outer = &mut world.objects[0];
        if let Shapes::Sphere(shape) = outer {
            shape.material.ambient = 1.0;
        }

        let inner = &mut world.objects[1];
        if let Shapes::Sphere(shape) = inner {
            shape.material.ambient = 1.0;
        }

        let ray = Ray {
            origin: Point::new(0.0, 0.0, 0.75),
            direction: Vector::new(0.0, 0.0, -1.0),
        };

        let inner = &world.objects[1];

        let color = world.color_at(ray);

        assert_eq!(color, inner.shape().material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::default();
        let point = Point::new(0.0, 10.0, 0.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = World::default();
        let point = Point::new(10.0, -10.0, 10.0);

        assert!(world.is_shadowed(point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = World::default();
        let point = Point::new(-20.0, 20.0, -20.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = World::default();
        let point = Point::new(-2.0, 2.0, -2.0);

        assert!(!world.is_shadowed(point));
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let s1 = Shapes::Sphere(Shape::default());

        let s2 = Shapes::Sphere(Shape {
            transform: Matrix::translation(0.0, 0.0, 10.0),
            ..Default::default()
        });

        let light = PointLight {
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

        let i = Intersection { t: 4.0, object: s2 };

        let comps = i.comps(ray);

        let color = world.shade_hit(comps);

        assert_eq!(
            color,
            Color {
                red: 0.1,
                green: 0.1,
                blue: 0.1
            }
        );
    }
}
