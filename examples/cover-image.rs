use raytracer::{
    camera::{self, consts::ImageResolution, Camera, CameraBuilder},
    color::{self, Color},
    light::{Light, PointLight},
    material::{self, Material},
    pattern::Pattern3D,
    shape::{Cube, Plane, Shape, ShapeBuilder, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

const RESOLUTION: ImageResolution = camera::consts::HD;

const WHITE_MATERIAL: Material = Material {
    pattern: Pattern3D::Solid(color::consts::WHITE),
    diffuse: 0.7,
    ambient: 0.1,
    specular: 0.0,
    reflectivity: 0.1,
    index_of_refraction: material::consts::VACUUM_INDEX_OF_REFRACTION,
    shininess: 200.0,
    transparency: 0.0,
};

const BLUE_MATERIAL: Material = Material {
    pattern: Pattern3D::Solid(Color {
        red: 0.537,
        green: 0.831,
        blue: 0.914,
    }),
    ..WHITE_MATERIAL
};

const RED_MATERIAL: Material = Material {
    pattern: Pattern3D::Solid(Color {
        red: 0.941,
        green: 0.322,
        blue: 0.388,
    }),
    ..WHITE_MATERIAL
};

const PURPLE_MATERIAL: Material = Material {
    pattern: Pattern3D::Solid(Color {
        red: 0.373,
        green: 0.404,
        blue: 0.550,
    }),
    ..WHITE_MATERIAL
};

fn main() {
    let std_transform =
        Transform::scaling(0.5, 0.5, 0.5).unwrap() * Transform::translation(1.0, -1.0, 1.0);

    let large_object = Transform::scaling(3.5, 3.5, 3.5).unwrap() * std_transform;
    let medium_object = Transform::scaling(3.0, 3.0, 3.0).unwrap() * std_transform;
    let small_object = Transform::scaling(2.0, 2.0, 2.0).unwrap() * std_transform;

    let backdrop = Shape::Plane(Plane::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::WHITE),
            ambient: 1.0,
            diffuse: 0.0,
            specular: 0.0,
            ..Default::default()
        },
        transform: Transform::translation(0.0, 0.0, 500.0)
            * Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    }));

    let sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.373,
                green: 0.404,
                blue: 0.55,
            }),
            diffuse: 0.2,
            ambient: 0.0,
            specular: 1.0,
            shininess: 200.0,
            reflectivity: 0.7,
            transparency: 0.7,
            index_of_refraction: 1.5,
        },
        transform: large_object,
    }));

    let mut objects = vec![backdrop, sphere];

    objects.extend([
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(4.0, 0.0, 0.0) * medium_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: BLUE_MATERIAL,
            transform: Transform::translation(8.5, 1.5, -0.5) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: RED_MATERIAL,
            transform: Transform::translation(0.0, 0.0, 4.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(4.0, 0.0, 4.0) * small_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: PURPLE_MATERIAL,
            transform: Transform::translation(7.5, 0.5, 4.0) * medium_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(-0.25, 0.25, 8.0) * medium_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: BLUE_MATERIAL,
            transform: Transform::translation(4.0, 1.0, 7.5) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: RED_MATERIAL,
            transform: Transform::translation(10.0, 2.0, 7.5) * medium_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(8.0, 2.0, 12.0) * small_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(20.0, 1.0, 9.0) * small_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: BLUE_MATERIAL,
            transform: Transform::translation(-0.5, -5.0, 0.25) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: RED_MATERIAL,
            transform: Transform::translation(4.0, -4.0, 0.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(8.5, -4.0, 0.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(0.0, -4.0, 4.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: PURPLE_MATERIAL,
            transform: Transform::translation(-0.5, -4.5, 8.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(0.0, -8.0, 4.0) * large_object,
        })),
        Shape::Cube(Cube::from(ShapeBuilder {
            material: WHITE_MATERIAL,
            transform: Transform::translation(-0.5, -8.5, 8.0) * large_object,
        })),
    ]);

    let main_light = Light::Point(PointLight {
        position: Point::new(50.0, 100.0, -50.0),
        intensity: color::consts::WHITE,
    });

    let secondary_light = Light::Point(PointLight {
        position: Point::new(-400.0, 50.0, -10.0),
        intensity: Color {
            red: 0.2,
            green: 0.2,
            blue: 0.2,
        },
    });

    let world = World {
        objects,
        lights: vec![main_light, secondary_light],
    };

    let camera = Camera::try_from(CameraBuilder {
        width: RESOLUTION.width,
        height: RESOLUTION.height,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(-6.0, 6.0, -10.0),
            Point::new(6.0, -4.0, 6.0),
            Vector::new(-0.45, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    let image = camera.render(&world).to_image();
    image.save("image.png").unwrap();
}
