use crate::matrix::Matrix;
use crate::tuple::Tuple;

pub type Transformation = Matrix<4, 4>;

pub fn translation(x: f64, y: f64, z: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][3] = x;
    transformation[1][3] = y;
    transformation[2][3] = z;

    transformation
}

pub fn scaling(x: f64, y: f64, z: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][0] = x;
    transformation[1][1] = y;
    transformation[2][2] = z;
    transformation[3][3] = 1.0;

    transformation
}

pub fn rotation_x(radians: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][0] = 1.0;
    transformation[1][1] = radians.cos();
    transformation[1][2] = -radians.sin();
    transformation[2][1] = radians.sin();
    transformation[2][2] = radians.cos();
    transformation[3][3] = 1.0;

    transformation
}

pub fn rotation_y(radians: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][0] = radians.cos();
    transformation[0][2] = radians.sin();
    transformation[1][1] = 1.0;
    transformation[2][0] = -radians.sin();
    transformation[2][2] = radians.cos();
    transformation[3][3] = 1.0;

    transformation
}

pub fn rotation_z(radians: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][0] = radians.cos();
    transformation[0][1] = -radians.sin();
    transformation[1][0] = radians.sin();
    transformation[1][1] = radians.cos();
    transformation[2][2] = 1.0;
    transformation[3][3] = 1.0;

    transformation
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transformation {
    let mut transformation = Matrix::identity();

    transformation[0][1] = xy;
    transformation[0][2] = xz;
    transformation[1][0] = yx;
    transformation[1][2] = yz;
    transformation[2][0] = zx;
    transformation[2][1] = zy;

    transformation
}

pub fn view(from: Tuple, to: Tuple, up: Tuple) -> Transformation {
    let forward = (to - from).normalize();
    let up = up.normalize();
    let left = forward.cross(up);
    let true_up = left.cross(forward);

    let orientation = Matrix::from([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    orientation * translation(-from.x, -from.y, -from.z)
}

impl Transformation {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        translation(x, y, z) * self
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        scaling(x, y, z) * self
    }

    pub fn rotate_x(self, radians: f64) -> Self {
        rotation_x(radians) * self
    }

    pub fn rotate_y(self, radians: f64) -> Self {
        rotation_y(radians) * self
    }

    pub fn rotate_z(self, radians: f64) -> Self {
        rotation_z(radians) * self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_a_transformation_matrix() {
        let translation_m = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(translation_m * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_transformation_matrix() {
        let translation_m = translation(5.0, -3.0, 2.0);
        let inv = translation_m.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let translation_m = translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(translation_m * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let scaling_m = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(scaling_m * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let scaling_m = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(scaling_m * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let scaling_m = scaling(2.0, 3.0, 4.0);
        let inv = scaling_m.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let scaling_m = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(scaling_m * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::FRAC_PI_4);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = rotation_y(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = rotation_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let shearing_m = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let shearing_m = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let shearing_m = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let shearing_m = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let shearing_m = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let shearing_m = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing_m * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation_m = rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling_m = scaling(5.0, 5.0, 5.0);
        let translation_m = translation(10.0, 5.0, 7.0);

        let p2 = rotation_m * p;

        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = scaling_m * p2;

        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = translation_m * p3;

        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation_m = rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling_m = scaling(5.0, 5.0, 5.0);
        let translation_m = translation(10.0, 5.0, 7.0);

        let t = translation_m * scaling_m * rotation_m;

        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn translation_identity_with_fluid_api_returns_translation_matrix() {
        let translation_m = Matrix::identity().translate(1.0, 1.0, 1.0);

        assert_eq!(translation_m, translation(1.0, 1.0, 1.0));
    }

    #[test]
    fn scaling_identity_with_fluid_api_returns_scaling_matrix() {
        let scaling_m = Matrix::identity().scale(1.0, 1.0, 1.0);

        assert_eq!(scaling_m, scaling(1.0, 1.0, 1.0));
    }

    #[test]
    fn rotating_identity_with_fluid_api_returns_rotation_matrix() {
        let rotation_x_m = Matrix::identity().rotate_x(std::f64::consts::FRAC_PI_2);
        let rotation_y_m = Matrix::identity().rotate_y(std::f64::consts::FRAC_PI_2);
        let rotation_z_m = Matrix::identity().rotate_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(rotation_x_m, rotation_x(std::f64::consts::FRAC_PI_2));
        assert_eq!(rotation_y_m, rotation_y(std::f64::consts::FRAC_PI_2));
        assert_eq!(rotation_z_m, rotation_z(std::f64::consts::FRAC_PI_2));
    }

    #[test]
    fn chained_transformations_with_fluid_api() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let transformation = Matrix::identity()
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view(from, to, up);

        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view(from, to, up);

        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = view(from, to, up);

        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let t = view(from, to, up);

        assert_eq!(
            t,
            Matrix::from([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000]
            ])
        );
    }
}
