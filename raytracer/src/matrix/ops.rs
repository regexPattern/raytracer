use std::ops::{Index, IndexMut, Mul};

use crate::{
    matrix::Matrix,
    tuple::{Point, Tuple, Vector},
};

impl<const M: usize, const N: usize> Index<usize> for Matrix<M, N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const M: usize, const N: usize> IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const M1: usize, const N: usize, const N2: usize> Mul<Matrix<N, N2>> for Matrix<M1, N> {
    type Output = Matrix<M1, N2>;

    fn mul(self, rhs: Matrix<N, N2>) -> Self::Output {
        let mut result = Matrix([[0.0; N2]; M1]);

        for row1 in 0..M1 {
            for col2 in 0..N2 {
                let mut value = 0.0;
                for col1 in 0..N {
                    value += self[row1][col1] * rhs[col1][col2];
                }

                result[row1][col2] = value;
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let result = self * Matrix([[rhs.x], [rhs.y], [rhs.z], [rhs.w]]);

        let x = result[0][0];
        let y = result[1][0];
        let z = result[2][0];
        let w = result[3][0];

        Tuple { x, y, z, w }
    }
}

impl Mul<Point> for Matrix<4, 4> {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(self * rhs.0)
    }
}

impl Mul<Vector> for Matrix<4, 4> {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector(self * rhs.0)
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::assert_approx;
    use crate::matrix;

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let M = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 5.6, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_approx!(M[0][0], 1.0);
        assert_approx!(M[0][3], 4.0);
        assert_approx!(M[1][0], 5.5);
        assert_approx!(M[1][2], 7.5);
        assert_approx!(M[2][2], 11.0);
        assert_approx!(M[3][0], 13.5);
        assert_approx!(M[3][2], 15.5);
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let M = Matrix([[-3.0, 5.0], [1.0, -2.0]]);

        assert_approx!(M[0][0], -3.0);
        assert_approx!(M[0][1], 5.0);
        assert_approx!(M[1][0], 1.0);
        assert_approx!(M[1][1], -2.0);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let M = Matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_approx!(M[0][0], -3.0);
        assert_approx!(M[1][1], -2.0);
        assert_approx!(M[2][2], 1.0);
    }

    #[test]
    fn mutating_a_matrix_value_by_indexing() {
        let mut M = Matrix([[1.0, 2.0], [3.0, 4.0]]);

        M[1][1] = 10.0;

        assert_approx!(M[1][1], 10.0);
    }

    #[test]
    fn multiplying_two_matrices() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let B = Matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            A * B,
            Matrix([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert_eq!(
            A * b,
            Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_point() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(A * p, Point::new(18.0, 24.0, 33.0));
    }

    #[test]
    fn a_matrix_multiplied_by_a_vector() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let v = Vector::new(1.0, 2.0, 3.0);

        assert_eq!(A * v, Vector::new(14.0, 22.0, 32.0));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let A = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(A * matrix::IDENTITY4X4, A);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let A = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let B = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let C = A * B;

        assert_eq!(C * B.inverse(), A);
    }
}
