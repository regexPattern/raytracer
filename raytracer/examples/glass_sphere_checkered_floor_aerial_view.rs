use raytracer::{
    camera::{self, consts::ImageResolution, Camera, CameraBuilder},
    color::{self, Color},
    light::PointLight,
    material::Material,
    pattern::{Pattern3D, Schema},
    scene::SceneProgress,
    shape::{
        plane::{Plane, PlaneBuilder},
        sphere::{Sphere, SphereBuilder},
        Shape,
    },
    transform::Transform,
    tuple::Point,
    world::World,
};

const RESOLUTION: ImageResolution = camera::consts::HD;

fn main() {
    let floor = Shape::Plane(Plane::from(PlaneBuilder {
        material: Material {
            pattern: Pattern3D::Checker(Schema::new(
                color::consts::WHITE,
                color::consts::BLACK,
                Default::default(),
            )),
            ..Default::default()
        },
        ..Default::default()
    }));

    let left_wall = Shape::Plane(Plane::from(PlaneBuilder {
        transform: Transform::rotation_z(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    }));

    let right_wall = Shape::Plane(Plane::from(PlaneBuilder {
        transform: Transform::rotation_x(std::f64::consts::FRAC_PI_2),
        ..Default::default()
    }));

    let glass_sphere = Shape::Sphere(Sphere::from(SphereBuilder {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.2,
                green: 0.2,
                blue: 0.3,
            }),
            transparency: 1.0,
            index_of_refraction: 1.35,
            specular: 0.01,
            reflectivity: 0.2,
            shininess: 400.0,
            ..Default::default()
        },
        transform: Transform::translation(6.0, 1.0, -6.0),
    }));

    let red_sphere = Shape::Sphere(Sphere::from(SphereBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::RED),
            ..Default::default()
        },
        transform: Transform::translation(4.0, 0.5, -6.0)
            * Transform::scaling(0.5, 0.5, 0.5).unwrap(),
    }));

    let blue_sphere = Shape::Sphere(Sphere::from(SphereBuilder {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.9,
            }),
            ..Default::default()
        },
        transform: Transform::scaling(0.75, 0.75, 0.75).unwrap()
            * Transform::translation(7.0, 1.0, -4.5),
    }));

    let green_sphere = Shape::Sphere(Sphere::from(SphereBuilder {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.5373,
                green: 0.6745,
                blue: 0.4627,
            }),
            ..Default::default()
        },
        transform: Transform::translation(3.0, 1.0, -3.0),
    }));

    let light = PointLight {
        position: Point::new(5.0, 5.0, -10.0),
        intensity: color::consts::WHITE,
    };

    let world = World {
        objects: vec![
            floor,
            left_wall,
            right_wall,
            glass_sphere,
            red_sphere,
            blue_sphere,
            green_sphere,
        ],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        image_width: RESOLUTION.width,
        image_height: RESOLUTION.height,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::rotation_x(std::f64::consts::FRAC_PI_2)
            * Transform::translation(-4.5, -12.0, 4.5),
    })
    .unwrap();

    let image = camera.render(&world, SceneProgress::Enable).to_image();
    image.save("image.png").unwrap();
}
