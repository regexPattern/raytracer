use crate::float;
use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};

const IDENTITY: Matrix<4, 4> = Matrix([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

#[derive(Clone, Debug)]
struct Matrix<const M: usize, const N: usize>([[f64; N]; M]);

impl Matrix<2, 2> {
    fn determinant(&self) -> f64 {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        a * d - b * c
    }
}

impl Matrix<3, 3> {
    fn determinant(&self) -> f64 {
        const FIXED_ROW: usize = 0;

        let mut result = 0.0;

        for col in 0..3 {
            result += self[FIXED_ROW][col] * self.cofactor(FIXED_ROW, col);
        }

        result
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix<2, 2> {
        let mut result = Matrix([[0.0; 2]; 2]);
        let mut result_rows = 0;

        for r in 0..3 {
            if r == row {
                continue;
            }

            let mut result_cols = 0;
            for c in 0..3 {
                if c == col {
                    continue;
                }

                result[result_rows][result_cols] = self[r][c];
                result_cols += 1;
            }

            result_rows += 1;
        }

        result
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Matrix<4, 4> {
    fn inverse(&self) -> Self {
        let mut result = Matrix([[0.0; 4]; 4]);
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                result[row][col] = self.cofactor(row, col) / determinant;
            }
        }

        result.transpose()
    }

    fn determinant(&self) -> f64 {
        const FIXED_ROW: usize = 0;

        let mut result = 0.0;

        for col in 0..4 {
            result += self[FIXED_ROW][col] * self.cofactor(FIXED_ROW, col);
        }

        result
    }

    fn transpose(&self) -> Self {
        let mut result = Matrix([[0.0; 4]; 4]);

        for row in 0..4 {
            for col in 0..4 {
                result[col][row] = self[row][col]
            }
        }

        result
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix<3, 3> {
        let mut result = Matrix([[0.0; 3]; 3]);
        let mut result_rows = 0;

        for r in 0..4 {
            if r == row {
                continue;
            }

            let mut result_cols = 0;
            for c in 0..4 {
                if c == col {
                    continue;
                }

                result[result_rows][result_cols] = self[r][c];
                result_cols += 1;
            }

            result_rows += 1;
        }

        result
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..M {
            for col in 0..N {
                if !float::approx(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }

        return true;
    }
}

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

impl<const M1: usize, const N: usize, const N2: usize> Mul<&Matrix<N, N2>> for &Matrix<M1, N> {
    type Output = Matrix<M1, N2>;

    fn mul(self, rhs: &Matrix<N, N2>) -> Self::Output {
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

impl Mul<Tuple> for &Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let result = self * &Matrix([[rhs.x], [rhs.y], [rhs.z], [rhs.w]]);

        let x = result[0][0];
        let y = result[1][0];
        let z = result[2][0];
        let w = result[3][0];

        Tuple { x, y, z, w }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::assert_approx;

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
    fn matrix_equality_with_identical_matrices() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let B = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(A, B);
    }

    #[test]
    fn matrix_equality_with_differente_matrices() {
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let B = Matrix([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(A, B);
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
            &A * &B,
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
            &A * b,
            Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0
            }
        );
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let A = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(&A * &IDENTITY, A);
    }

    #[test]
    fn transposing_a_matrix() {
        let A = Matrix([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        assert_eq!(
            A.transpose(),
            Matrix([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ])
        );
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert_eq!(IDENTITY.transpose(), IDENTITY);
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let A = Matrix([[1.0, 5.0], [-3.0, 2.0]]);

        assert_approx!(A.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let A = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        assert_eq!(A.submatrix(0, 2), Matrix([[-3.0, 2.0], [0.0, 6.0],]));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let A = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            A.submatrix(2, 1),
            Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0],])
        );
    }

    #[test]
    fn calculating_the_minor_of_a_3x3_matrix() {
        let A = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let B = A.submatrix(1, 0);

        assert_approx!(B.determinant(), 25.0);
        assert_approx!(A.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_the_cofactor_of_a_3x3_matrix() {
        let A = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_approx!(A.minor(0, 0), -12.0);
        assert_approx!(A.cofactor(0, 0), -12.0);
        assert_approx!(A.minor(1, 0), 25.0);
        assert_approx!(A.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let A = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_approx!(A.cofactor(0, 0), 56.0);
        assert_approx!(A.cofactor(0, 1), 12.0);
        assert_approx!(A.cofactor(0, 2), -46.0);
        assert_approx!(A.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let A = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx!(A.cofactor(0, 0), 690.0);
        assert_approx!(A.cofactor(0, 1), 447.0);
        assert_approx!(A.cofactor(0, 2), 210.0);
        assert_approx!(A.cofactor(0, 3), 51.0);
        assert_approx!(A.determinant(), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let A = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_approx!(A.determinant(), -2120.0, "Matrix `A` is invertible");
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let A = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_approx!(A.determinant(), 0.0, "Matrix `A` is not invertible");
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let A = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let B = A.inverse();

        assert_approx!(A.determinant(), 532.0);
        assert_approx!(A.cofactor(2, 3), -160.0);
        assert_approx!(B[3][2], -160.0 / 532.0);
        assert_approx!(A.cofactor(3, 2), 105.0);
        assert_approx!(B[2][3], 105.0 / 532.0);
        assert_eq!(
            B,
            Matrix([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let A = Matrix([
            [8.0, -5.0, 9., 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        assert_eq!(
            A.inverse(),
            Matrix([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ])
        );
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let A = Matrix([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        assert_eq!(
            A.inverse(),
            Matrix([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ])
        );
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

        let C = &A * &B;

        assert_eq!(&C * &B.inverse(), A);
    }
}
