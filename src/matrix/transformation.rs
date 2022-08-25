use crate::matrix::{self, Matrix};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4.identity();

    matrix[0][3] = x;
    matrix[1][3] = y;
    matrix[2][3] = z;

    matrix
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4;

    matrix[0][0] = x;
    matrix[1][1] = y;
    matrix[2][2] = z;
    matrix[3][3] = 1.0;

    matrix
}

pub fn rotation_x(radians: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4;

    matrix[0][0] = 1.0;
    matrix[1][1] = radians.cos();
    matrix[1][2] = -radians.sin();
    matrix[2][1] = radians.sin();
    matrix[2][2] = radians.cos();
    matrix[3][3] = 1.0;

    matrix
}

pub fn rotation_y(radians: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4;

    matrix[0][0] = radians.cos();
    matrix[0][2] = radians.sin();
    matrix[1][1] = 1.0;
    matrix[2][0] = -radians.sin();
    matrix[2][2] = radians.cos();
    matrix[3][3] = 1.0;

    matrix
}

pub fn rotation_z(radians: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4;

    matrix[0][0] = radians.cos();
    matrix[0][1] = -radians.sin();
    matrix[1][0] = radians.sin();
    matrix[1][1] = radians.cos();
    matrix[2][2] = 1.0;
    matrix[3][3] = 1.0;

    matrix
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4, 4> {
    let mut matrix = matrix::MATRIX_4X4.identity();

    matrix[0][1] = xy;
    matrix[0][2] = xz;
    matrix[1][0] = yx;
    matrix[1][2] = yz;
    matrix[2][0] = zx;
    matrix[2][1] = zy;

    matrix
}

impl Matrix<4, 4> {
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
    use crate::matrix::{self, Matrix};
    use crate::tuple::Tuple;

    #[test]
    fn multiplying_by_a_transformation_matrix() {
        let translation = super::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(translation * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_transformation_matrix() {
        let translation = super::translation(5.0, -3.0, 2.0);
        let inv = translation.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let translation = super::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(translation * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let scaling = super::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(scaling * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let scaling = super::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(scaling * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let scaling = super::scaling(2.0, 3.0, 4.0);
        let inv = scaling.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let scaling = super::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(scaling * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = super::rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = super::rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = super::rotation_x(std::f64::consts::FRAC_PI_4);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = super::rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = super::rotation_y(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = super::rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = super::rotation_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let shearing = super::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let shearing = super::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let shearing = super::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let shearing = super::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let shearing = super::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let shearing = super::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(shearing * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation = super::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = super::scaling(5.0, 5.0, 5.0);
        let translation = super::translation(10.0, 5.0, 7.0);

        let p2 = rotation * p;

        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = scaling * p2;

        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = translation * p3;

        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation = super::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = super::scaling(5.0, 5.0, 5.0);
        let translation = super::translation(10.0, 5.0, 7.0);

        let t = translation * scaling * rotation;

        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn translation_identity_with_fluid_api_returns_translation_matrix() {
        let translation = matrix::MATRIX_4X4.identity().translate(1.0, 1.0, 1.0);

        assert_eq!(translation, super::translation(1.0, 1.0, 1.0));
    }

    #[test]
    fn scaling_identity_with_fluid_api_returns_scaling_matrix() {
        let scaling = matrix::MATRIX_4X4.identity().scale(1.0, 1.0, 1.0);

        assert_eq!(scaling, super::scaling(1.0, 1.0, 1.0));
    }

    #[test]
    fn rotating_identity_with_fluid_api_returns_rotation_matrix() {
        let rotation_x = matrix::MATRIX_4X4
            .identity()
            .rotate_x(std::f64::consts::FRAC_PI_2);
        let rotation_y = matrix::MATRIX_4X4
            .identity()
            .rotate_y(std::f64::consts::FRAC_PI_2);
        let rotation_z = matrix::MATRIX_4X4
            .identity()
            .rotate_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(rotation_x, super::rotation_x(std::f64::consts::FRAC_PI_2));
        assert_eq!(rotation_y, super::rotation_y(std::f64::consts::FRAC_PI_2));
        assert_eq!(rotation_z, super::rotation_z(std::f64::consts::FRAC_PI_2));
    }

    #[test]
    fn chained_transformations_with_fluid_api() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let transformation = Matrix([[0.0; 4]; 4])
            .identity()
            .rotate_x(std::f64::consts::FRAC_PI_2)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert_eq!(transformation * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
