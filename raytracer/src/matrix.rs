use std::ops::{Index, IndexMut, Mul};

use crate::{float, tuple::Tuple};

pub mod consts {
    use super::Matrix;

    pub const IDENTITY_4X4: Matrix<4, 4> = Matrix([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);
}

#[derive(Debug, PartialEq)]
pub struct NonInvertibleMatrixError;

#[derive(Copy, Clone, Debug)]
pub struct Matrix<const M: usize, const N: usize>(pub [[f64; N]; M]);

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                if !float::approx(self[i][j], other[i][j]) {
                    return false;
                }
            }
        }

        true
    }
}

impl Matrix<2, 2> {
    fn determinant(self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

fn populate_submatrix_aux<const N1: usize, const N2: usize>(
    origin: &Matrix<N1, N1>,
    dest: &mut Matrix<N2, N2>,
    target_row: usize,
    target_col: usize,
) {
    let mut rows = 0;
    let mut cols = 0;

    for i in 0..N1 {
        if i == target_row {
            continue;
        }

        for j in 0..N1 {
            if j == target_col {
                continue;
            }

            dest[rows][cols] = origin[i][j];
            cols += 1;
        }

        rows += 1;
        cols = 0;
    }
}

impl Matrix<3, 3> {
    fn submatrix(self, row: usize, col: usize) -> Matrix<2, 2> {
        let mut submatrix = Matrix([[0.0; 2]; 2]);

        populate_submatrix_aux(&self, &mut submatrix, row, col);

        submatrix
    }

    fn minor(self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(self, row: usize, col: usize) -> f64 {
        (-1_f64).powi((row + col) as i32) * self.minor(row, col)
    }

    fn determinant(self) -> f64 {
        let fixed_row = self[0];
        fixed_row
            .iter()
            .enumerate()
            .fold(0.0, |acc, (col, value)| acc + value * self.cofactor(0, col))
    }
}

impl Matrix<4, 4> {
    pub fn transpose(self) -> Self {
        let mut result = Self([[0.0; 4]; 4]);

        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = self[j][i];
            }
        }

        result
    }

    fn submatrix(self, row: usize, col: usize) -> Matrix<3, 3> {
        let mut submatrix = Matrix([[0.0; 3]; 3]);

        populate_submatrix_aux(&self, &mut submatrix, row, col);

        submatrix
    }

    fn minor(self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(self, row: usize, col: usize) -> f64 {
        (-1_f64).powi((row + col) as i32) * self.minor(row, col)
    }

    fn determinant(self) -> f64 {
        let fixed_row = self[0];
        fixed_row
            .iter()
            .enumerate()
            .fold(0.0, |acc, (col, value)| acc + value * self.cofactor(0, col))
    }

    pub fn inverse(self) -> Result<Self, NonInvertibleMatrixError> {
        let det = self.determinant();
        let mut inv = Self([[0.0; 4]; 4]);

        if float::approx(det, 0.0) {
            return Err(NonInvertibleMatrixError);
        }

        for i in 0..4 {
            for j in 0..4 {
                inv[j][i] = self.cofactor(i, j) / det;
            }
        }

        Ok(inv)
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

impl<const M: usize, const N: usize, const O: usize> Mul<Matrix<N, O>> for Matrix<M, N> {
    type Output = Matrix<M, O>;

    fn mul(self, rhs: Matrix<N, O>) -> Self::Output {
        let mut result = Matrix([[0.0; O]; M]);

        for i in 0..N {
            for j in 0..O {
                result[i][j] = (0..M).fold(0.0, |acc, k| acc + self.0[i][k] * rhs.0[k][j]);
            }
        }

        result
    }
}

impl Mul<Tuple> for Matrix<4, 4> {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let col = Matrix([[rhs.x], [rhs.y], [rhs.z], [rhs.w]]);
        let result = self * col;

        let x = result[0][0];
        let y = result[1][0];
        let z = result[2][0];
        let w = result[3][0];

        Tuple { x, y, z, w }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_approx;

    use super::*;

    fn matrix_is_invertible(m: Matrix<4, 4>) -> bool {
        !float::approx(m.determinant(), 0.0)
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let m = Matrix([[-3.0, 5.0], [1.0, -2.0]]);

        assert_approx!(m[0][0], -3.0);
        assert_approx!(m[0][1], 5.0);
        assert_approx!(m[1][0], 1.0);
        assert_approx!(m[1][1], -2.0);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let m = Matrix([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_approx!(m[0][0], -3.0);
        assert_approx!(m[1][1], -2.0);
        assert_approx!(m[2][2], 1.0);
    }

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_approx!(m[0][0], 1.0);
        assert_approx!(m[0][3], 4.0);
        assert_approx!(m[1][0], 5.5);
        assert_approx!(m[1][2], 7.5);
        assert_approx!(m[2][2], 11.0);
        assert_approx!(m[3][0], 13.5);
        assert_approx!(m[3][2], 15.5);
    }

    #[test]
    fn comparing_matrices() {
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
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn multiplying_two_matrices() {
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

        assert_eq!(
            m2 * m1,
            Matrix([
                [36.0, 30.0, 24.0, 18.0],
                [17.0, 22.0, 27.0, 32.0],
                [98.0, 94.0, 90.0, 86.0],
                [114.0, 102.0, 90.0, 78.0],
            ])
        );
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let m = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert_eq!(
            m * t,
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
        let m = Matrix([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(m * super::consts::IDENTITY_4X4, m);
        assert_eq!(
            m * super::consts::IDENTITY_4X4,
            super::consts::IDENTITY_4X4 * m
        );
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };

        assert_eq!(super::consts::IDENTITY_4X4 * t, t);
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
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let identity = Matrix([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(identity.transpose(), identity);
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let m = Matrix([[1.0, 5.0], [-3.0, 2.0]]);

        assert_approx!(m.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let m = Matrix([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        assert_eq!(m.submatrix(0, 2), Matrix([[-3.0, 2.0], [0.0, 6.0]]));
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let m = Matrix([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert_eq!(
            m.submatrix(2, 1),
            Matrix([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0],])
        );
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        let b = m.submatrix(1, 0);

        assert_approx!(b.determinant(), 25.0);
        assert_approx!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let m = Matrix([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_approx!(m.minor(0, 0), -12.0);
        assert_approx!(m.cofactor(0, 0), -12.0);
        assert_approx!(m.minor(1, 0), 25.0);
        assert_approx!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let m = Matrix([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_approx!(m.cofactor(0, 0), 56.0);
        assert_approx!(m.cofactor(0, 1), 12.0);
        assert_approx!(m.cofactor(0, 2), -46.0);
        assert_approx!(m.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let m = Matrix([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_approx!(m.cofactor(0, 0), 690.0);
        assert_approx!(m.cofactor(0, 1), 447.0);
        assert_approx!(m.cofactor(0, 2), 210.0);
        assert_approx!(m.cofactor(0, 3), 51.0);
        assert_approx!(m.determinant(), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let m = Matrix([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_approx!(m.determinant(), -2120.0);
        assert!(matrix_is_invertible(m));
    }

    #[test]
    fn testing_a_non_invertible_matrix_for_invertibility() {
        let m = Matrix([[0.0; 4]; 4]);

        assert_approx!(m.determinant(), 0.0);
        assert!(!matrix_is_invertible(m));
    }

    #[test]
    fn calculating_the_inverse_of_an_invertible_matrix() {
        let m = Matrix([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let inv = m.inverse().unwrap();

        assert_approx!(m.determinant(), 532.0);
        assert_approx!(m.cofactor(2, 3), -160.0);
        assert_approx!(inv[3][2], -160.0 / 532.0);
        assert_approx!(m.cofactor(3, 2), 105.0);
        assert_approx!(inv[2][3], 105.0 / 532.0);
        assert_eq!(
            inv,
            Matrix([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639],
            ])
        );
    }

    #[test]
    fn trying_to_calculate_the_inverse_of_a_non_invertible_matrix() {
        let m = Matrix([[0.0; 4]; 4]);

        assert_eq!(m.inverse(), Err(NonInvertibleMatrixError));
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let m = Matrix([
            [8.0, -5.0, 9., 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);

        assert_eq!(
            m.inverse(),
            Ok(Matrix([
                [-0.15385, -0.15385, -0.28205, -0.53846],
                [-0.07692, 0.12308, 0.02564, 0.03077],
                [0.35897, 0.35897, 0.43590, 0.92308],
                [-0.69231, -0.69231, -0.76923, -1.92308],
            ]))
        );
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let m = Matrix([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);

        assert_eq!(
            m.inverse(),
            Ok(Matrix([
                [-0.04074, -0.07778, 0.14444, -0.22222],
                [-0.07778, 0.03333, 0.36667, -0.33333],
                [-0.02901, -0.14630, -0.10926, 0.12963],
                [0.17778, 0.06667, -0.26667, 0.33333],
            ]))
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

        let m3 = m1 * m2;

        assert_eq!(m3 * m2.inverse().unwrap(), m1);
    }
}
