use raytracer::{
    camera::{self, consts::ImageResolution, Camera, CameraBuilder},
    color::{self, Color},
    light::{AreaLight, AreaLightBuilder, Light},
    material::Material,
    pattern::{Pattern3D, Pattern3DSpec},
    shape::{Plane, Shape, ShapeBuilder, Sphere},
    transform::Transform,
    tuple::{Point, Vector},
    world::World,
};

const RESOLUTION: ImageResolution = camera::consts::QHD;

fn main() {
    let floor = Shape::Plane(Plane::default());

    let wall_material = Material {
        pattern: Pattern3D::Checker(Pattern3DSpec::new(
            color::consts::WHITE,
            color::consts::BLACK,
            Default::default(),
        )),
        ..Default::default()
    };

    let left_wall = Shape::Plane(Plane::from(ShapeBuilder {
        material: wall_material.clone(),
        transform: Transform::translation(0.0, 1.0, 0.0)
            * Transform::rotation_z(std::f64::consts::FRAC_PI_2),
    }));

    let right_wall = Shape::Plane(Plane::from(ShapeBuilder {
        material: wall_material.clone(),
        transform: Transform::rotation_x(std::f64::consts::FRAC_PI_2),
    }));

    let metallic_sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
        material: Material {
            reflectivity: 0.25,
            pattern: Pattern3D::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
            }),
            ..Default::default()
        },
        transform: Transform::translation(4.0, 1.0, -4.0),
    }));

    let red_sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(color::consts::RED),
            ..Default::default()
        },
        transform: Transform::translation(4.0, 0.5, -6.0)
            * Transform::scaling(0.5, 0.5, 0.5).unwrap(),
    }));

    let blue_sphere = Shape::Sphere(Sphere::from(ShapeBuilder {
        material: Material {
            pattern: Pattern3D::Solid(Color {
                red: 0.5,
                green: 0.5,
                blue: 0.9,
            }),
            ..Default::default()
        },
        transform: Transform::translation(6.0, 0.25, -4.5)
            * Transform::scaling(0.25, 0.25, 0.25).unwrap(),
    }));

    let light = Light::Area(AreaLight::from(AreaLightBuilder {
        corner: Point::new(5.0, 5.0, -10.0),
        horizontal_dir: Vector::new(4.0, 0.0, 0.0),
        horizontal_cells: 8,
        vertical_dir: Vector::new(0.0, 4.0, 0.0),
        vertical_cells: 8,
        intensity: color::consts::WHITE,
    }));

    let world = World {
        objects: vec![
            floor,
            left_wall,
            right_wall,
            metallic_sphere,
            red_sphere,
            blue_sphere,
        ],
        lights: vec![light],
    };

    let camera = Camera::try_from(CameraBuilder {
        width: RESOLUTION.width,
        height: RESOLUTION.height,
        field_of_view: std::f64::consts::FRAC_PI_3,
        transform: Transform::view(
            Point::new(10.0, 3.0, -10.0),
            Point::new(0.0, 0.0, 0.0),
            Vector::new(0.0, 1.0, 0.0),
        )
        .unwrap(),
    })
    .unwrap();

    let image = camera.render(&world).to_image();
    image.save("image.png").unwrap();
}
