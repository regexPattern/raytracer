use crate::float;

mod ops;
mod transformations;

pub const IDENTITY4X4: Matrix<4, 4> = Matrix([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const M: usize, const N: usize>([[f64; N]; M]);

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for row in 0..M {
            for col in 0..N {
                if !float::approx(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

impl Matrix<2, 2> {
    fn determinant(self) -> f64 {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];

        a * d - b * c
    }
}

impl Matrix<3, 3> {
    fn cofactor(self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(self) -> f64 {
        const FIXED_ROW: usize = 0;

        let mut result = 0.0;

        for col in 0..3 {
            result += self[FIXED_ROW][col] * self.cofactor(FIXED_ROW, col);
        }

        result
    }

    fn minor(self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn submatrix(self, row: usize, col: usize) -> Matrix<2, 2> {
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
}

impl Matrix<4, 4> {
    fn cofactor(self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(self) -> f64 {
        const FIXED_ROW: usize = 0;

        let mut result = 0.0;

        for col in 0..4 {
            result += self[FIXED_ROW][col] * self.cofactor(FIXED_ROW, col);
        }

        result
    }

    pub fn inverse(self) -> Self {
        let mut result = Self([[0.0; 4]; 4]);
        let determinant = self.determinant();

        for row in 0..4 {
            for col in 0..4 {
                result[row][col] = self.cofactor(row, col) / determinant;
            }
        }

        result.transpose()
    }

    fn minor(self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn submatrix(self, row: usize, col: usize) -> Matrix<3, 3> {
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

    pub fn transpose(self) -> Self {
        let mut result = Self([[0.0; 4]; 4]);

        for row in 0..4 {
            for col in 0..4 {
                result[col][row] = self[row][col];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use crate::assert_approx;

    use super::*;

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
        assert_eq!(IDENTITY4X4.transpose(), IDENTITY4X4);
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
}
