use crate::matrix::Matrix;
use crate::tuple::Tuple;
use std::ops::Mul;

pub struct Transformation;

impl Matrix<4, 4> {
    pub fn translation(self, x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        let mut matrix = self.identity();

        matrix[0][3] = x;
        matrix[1][3] = y;
        matrix[2][3] = z;

        matrix
    }
}

impl Transformation {
    pub fn translation(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        let mut matrix = Matrix([[0.0; 4]; 4]).identity();

        matrix[0][3] = x;
        matrix[1][3] = y;
        matrix[2][3] = z;

        matrix
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Matrix<4, 4> {
        let mut matrix = Matrix([[0.0; 4]; 4]);

        matrix[0][0] = x;
        matrix[1][1] = y;
        matrix[2][2] = z;
        matrix[3][3] = 1.0;

        matrix
    }

    pub fn rotation_x(radians: f64) -> Matrix<4, 4> {
        let mut matrix = Matrix([[0.0; 4]; 4]);

        matrix[0][0] = 1.0;
        matrix[1][1] = radians.cos();
        matrix[1][2] = -radians.sin();
        matrix[2][1] = radians.sin();
        matrix[2][2] = radians.cos();
        matrix[3][3] = 1.0;

        matrix
    }

    pub fn rotation_y(radians: f64) -> Matrix<4, 4> {
        // TODO: Deberia implementar `Default` para Matrix;
        let mut matrix = Matrix([[0.0; 4]; 4]);

        matrix[0][0] = radians.cos();
        matrix[0][2] = radians.sin();
        matrix[1][1] = 1.0;
        matrix[2][0] = -radians.sin();
        matrix[2][2] = radians.cos();
        matrix[3][3] = 1.0;

        matrix
    }

    pub fn rotation_z(radians: f64) -> Matrix<4, 4> {
        let mut matrix = Matrix([[0.0; 4]; 4]);

        matrix[0][0] = radians.cos();
        matrix[0][1] = -radians.sin();
        matrix[1][0] = radians.sin();
        matrix[1][1] = radians.cos();
        matrix[2][2] = 1.0;
        matrix[3][3] = 1.0;

        matrix
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix<4, 4> {
        let mut matrix = Matrix([[0.0; 4]; 4]).identity();

        matrix[0][1] = xy;
        matrix[0][2] = xz;
        matrix[1][0] = yx;
        matrix[1][2] = yz;
        matrix[2][0] = zx;
        matrix[2][1] = zy;

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplying_by_a_transformation_matrix() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_transformation_matrix() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let inv = t.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(t * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_a_point() {
        let s = Transformation::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(s * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_a_vector() {
        let s = Transformation::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(s * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let s = Transformation::scaling(2.0, 3.0, 4.0);
        let inv = s.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let s = Transformation::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotation_x(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transformation::rotation_x(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotation_x(std::f64::consts::FRAC_PI_4);
        let inv = half_quarter.inverse();

        assert_eq!(
            inv * p,
            Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt()) / 2.0)
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Transformation::rotation_y(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transformation::rotation_y(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transformation::rotation_z(std::f64::consts::FRAC_PI_4);
        let full_quarter = Transformation::rotation_z(std::f64::consts::FRAC_PI_2);

        assert_eq!(
            half_quarter * p,
            Tuple::point(-2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0)
        );

        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let s = Transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let s = Transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_x() {
        let s = Transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let s = Transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let s = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let s = Transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(s * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let rotation = Transformation::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = Transformation::scaling(5.0, 5.0, 5.0);
        let translation = Transformation::translation(10.0, 5.0, 7.0);

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
        let rotation = Transformation::rotation_x(std::f64::consts::FRAC_PI_2);
        let scaling = Transformation::scaling(5.0, 5.0, 5.0);
        let translation = Transformation::translation(10.0, 5.0, 7.0);

        let t = translation * scaling * rotation;

        assert_eq!(t * p, Tuple::point(15.0, 0.0, 7.0));
    }
}
