use std::ops::{Index, IndexMut, Mul};

use crate::tuple::Tuple;
use crate::utils;

mod transformations;

pub use transformations::Transformation;

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const M: usize, const N: usize>([[f64; N]; M]);

impl<const N: usize> Matrix<N, N> {
    fn identity(&self) -> Matrix<N, N> {
        let mut identity = Matrix([[0.0; N]; N]);
        for n in 0..N {
            identity[n][n] = 1.0;
        }
        identity
    }

    fn transpose(self) -> Matrix<N, N> {
        let mut transposed = Matrix([[0.0; N]; N]);
        for col in 0..N {
            for row in 0..N {
                transposed[col][row] = self.0[row][col];
            }
        }
        transposed
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

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Matrix<M, N>) -> bool {
        for row in 0..M {
            for col in 0..N {
                if !utils::approximately_eq(self.0[row][col], other.0[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const M1: usize, const N1: usize, const N2: usize> Mul<Matrix<N1, N2>> for Matrix<M1, N1> {
    type Output = Matrix<M1, N2>;

    fn mul(self, rhs: Matrix<N1, N2>) -> Self::Output {
        let mut result = Matrix([[0.0; N2]; M1]);

        for row1 in 0..M1 {
            for col2 in 0..N2 {
                let mut value = 0.0;
                for col1 in 0..N1 {
                    value += self.0[row1][col1] * rhs.0[col1][col2]
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
        let column_matrix = Matrix([[rhs.x], [rhs.y], [rhs.z], [rhs.w]]);
        let result = self * column_matrix;

        Tuple::new(result[0][0], result[1][0], result[2][0], result[3][0])
    }
}

impl Matrix<2, 2> {
    fn determinant(&self) -> f64 {
        (self.0[0][0] * self.0[1][1]) - (self.0[0][1] * self.0[1][0])
    }
}

impl Matrix<3, 3> {
    fn cofactor(&self, removed_row: usize, removed_col: usize) -> f64 {
        let minor = self.minor(removed_row, removed_col);
        if (removed_row + removed_col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        let static_row = 0;

        for (col, elem) in self.0[static_row].iter().enumerate() {
            determinant += elem * self.cofactor(static_row, col);
        }

        determinant
    }

    fn minor(&self, removed_row: usize, removed_col: usize) -> f64 {
        self.submatrix(removed_row, removed_col).determinant()
    }

    fn submatrix(&self, removed_row: usize, removed_col: usize) -> Matrix<2, 2> {
        let mut submatrix = Matrix([[0.0; 2]; 2]);
        let mut skipped_rows = 0;

        for row in 0..2 {
            if row == removed_row {
                skipped_rows += 1;
            }

            let mut skipped_cols = 0;
            for col in 0..2 {
                if col == removed_col {
                    skipped_cols += 1;
                }

                submatrix[row][col] = self.0[row + skipped_rows][col + skipped_cols];
            }
        }

        submatrix
    }
}

impl Matrix<4, 4> {
    fn cofactor(&self, removed_row: usize, removed_col: usize) -> f64 {
        let minor = self.minor(removed_row, removed_col);
        if (removed_row + removed_col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        let static_row = 0;

        for (col, elem) in self.0[static_row].iter().enumerate() {
            determinant += elem * self.cofactor(static_row, col);
        }

        determinant
    }

    fn inverse(&self) -> Matrix<4, 4> {
        let determinant = self.determinant();
        let mut cofactors = Matrix([[0.0; 4]; 4]);

        for row in 0..4 {
            for col in 0..4 {
                cofactors[row][col] = self.cofactor(row, col) / determinant;
            }
        }

        cofactors.transpose()
    }

    fn is_inversible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn minor(&self, removed_row: usize, removed_col: usize) -> f64 {
        self.submatrix(removed_row, removed_col).determinant()
    }

    fn submatrix(&self, removed_row: usize, removed_col: usize) -> Matrix<3, 3> {
        let mut submatrix = Matrix([[0.0; 3]; 3]);
        let mut skipped_rows = 0;

        for row in 0..3 {
            if row == removed_row {
                skipped_rows += 1;
            }

            let mut skipped_cols = 0;
            for col in 0..3 {
                if col == removed_col {
                    skipped_cols += 1;
                }

                submatrix[row][col] = self.0[row + skipped_rows][col + skipped_cols];
            }
        }

        submatrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let m = Matrix([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(m[0], [-3.0, 5.0]);
        assert_eq!(m[1], [1.0, -2.0]);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let m = Matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [1.0, 1.0, 1.0]]);

        assert_eq!(m[0], [-3.0, 5.0, 0.0]);
        assert_eq!(m[1], [1.0, -2.0, -7.0]);
        assert_eq!(m[2], [1.0, 1.0, 1.0]);
    }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0], [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(m[1], [5.5, 6.5, 7.5, 8.5]);
        assert_eq!(m[2], [9.0, 10.0, 11.0, 12.0]);
        assert_eq!(m[3], [13.5, 14.5, 15.5, 16.5]);
    }

    #[test]
    fn constructing_and_inspecting_an_mxn_matrix() {
        let m = Matrix([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]);

        assert_eq!(m[0], [1.0, 2.0, 3.0]);
        assert_eq!(m[1], [4.0, 5.0, 6.0]);
    }

    #[test]
    fn comparing_squared_matrices() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m3 = Matrix([
            [1.5, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn comparing_column_matrices() {
        let m1 = Matrix([[1.0], [2.0], [3.0]]);
        let m2 = Matrix([[1.0], [2.0], [3.0]]);
        let m3 = Matrix([[1.5], [2.0], [3.0]]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn mutating_matrix_values() {
        let mut m = Matrix([[1.0, 2.0], [2.0, 4.0]]);

        m[1][0] = 3.0;

        assert_eq!(m[1][0], 3.0);
    }

    #[test]
    fn multiplying_4x4_matrices() {
        let m1 = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m2 = Matrix([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            m1 * m2,
            Matrix([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn multiplying_4x4_matrix_by_4x1_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let v = Matrix([[1.0], [2.0], [3.0], [1.0]]);

        assert_eq!(m * v, Matrix([[18.0], [24.0], [33.0], [1.0]]));
    }

    #[test]
    fn getting_the_identity_matrix_for_squared_matrices() {
        let m1 = Matrix([[5.0]]);
        let m2 = Matrix([[3.0, 4.0], [5.0, 6.0]]);
        let m3 = Matrix([[11.0, 12.0, 13.0], [14.0, 15.0, 16.0], [17.0, 18.0, 19.0]]);

        assert_eq!(m1.identity(), Matrix([[1.0]]));
        assert_eq!(m2.identity(), Matrix([[1.0, 0.0], [0.0, 1.0]]));
        assert_eq!(
            m3.identity(),
            Matrix([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );
    }

    #[test]
    fn multiplying_by_the_identity_matrix() {
        let m1 = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        let i = Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let m2 = Matrix([[2.0], [2.0], [2.0], [2.0]]);

        assert_eq!(m1 * i, m1);
        assert_eq!(i * m2, m2);
    }

    #[test]
    fn transposing_a_matrix() {
        let m = Matrix([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        assert_eq!(
            m.transpose(),
            Matrix([
                [0.0, 9.0, 1.0, 0.0],
                [9.0, 8.0, 8.0, 0.0],
                [3.0, 0.0, 5.0, 5.0],
                [0.0, 8.0, 3.0, 8.0],
            ])
        );

        assert_eq!(
            m.transpose().identity(),
            Matrix([
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }

    #[test]
    fn calculating_determinant_of_2x2_matrix() {
        let m = Matrix([[1.0, 5.0], [-3.0, 2.0]]);

        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn get_submatrices_of_squared_matrices() {
        let m1 = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let m2 = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(m1.submatrix(0, 2), Matrix([[-3.0, 2.0], [0.0, 6.0]]));
        assert_eq!(m1.submatrix(2, 2), Matrix([[1.0, 5.0], [-3.0, 2.0]]));
        assert_eq!(
            m2.submatrix(2, 1),
            Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn calculating_minor_of_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(m.submatrix(0, 0).determinant(), -12.0);
        assert_eq!(m.minor(0, 0), -12.0);

        assert_eq!(m.submatrix(1, 0).determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_cofactor_of_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);

        assert_eq!(m.minor(0, 1), 52.0);
        assert_eq!(m.cofactor(0, 1), -52.0);

        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_determinant_of_3x3_matrix() {
        let m1 = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        let m2 = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_eq!(m1.cofactor(0, 0), 56.0);
        assert_eq!(m1.cofactor(0, 1), 12.0);
        assert_eq!(m1.cofactor(0, 2), -46.0);
        assert_eq!(m1.determinant(), -196.0);

        assert_eq!(m2.cofactor(0, 0), 690.0);
        assert_eq!(m2.cofactor(0, 1), 447.0);
        assert_eq!(m2.cofactor(0, 2), 210.0);
        assert_eq!(m2.cofactor(0, 3), 51.0);
        assert_eq!(m2.determinant(), -4071.0);
    }

    #[test]
    fn testing_matrix_inversibility() {
        let m1 = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let m2 = Matrix([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(m1.determinant(), -2120.0);
        assert!(m1.is_inversible());

        assert_eq!(m2.determinant(), 0.0);
        assert!(!m2.is_inversible());
    }

    #[test]
    fn calculating_inverse_of_matrix() {
        let m = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let inverse = m.inverse();

        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_eq!(inverse[3][2], -160.0 / 532.0);
        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(inverse[2][3], 105.0 / 532.0);
        assert_eq!(
            inverse,
            Matrix([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let m1 = Matrix([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let m2 = Matrix([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let product = m1 * m2;

        assert_eq!(product * m2.inverse(), m1);
    }
}
