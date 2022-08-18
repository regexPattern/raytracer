use crate::utils;
use std::ops::{Index, IndexMut, Mul};

// TODO: Ver si puedo meter todo esto en un solo tipo con dos generics.
#[derive(Debug)]
struct VectorMatrix<const N: usize>([f64; N]);

#[derive(Debug)]
struct Matrix<const N: usize>([[f64; N]; N]);

impl<const N: usize> From<[[f64; N]; N]> for Matrix<N> {
    fn from(values: [[f64; N]; N]) -> Matrix<N> {
        Matrix(values)
    }
}

impl<const N: usize> Index<usize> for Matrix<N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Matrix<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> PartialEq for Matrix<N> {
    fn eq(&self, other: &Matrix<N>) -> bool {
        for row in 0..N {
            for col in 0..N {
                if !utils::approximately_eq(self.0[row][col], other.0[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const N: usize> Mul for Matrix<N> {
    type Output = Matrix<N>;

    fn mul(self, rhs: Matrix<N>) -> Self::Output {
        let mut result = Matrix::from([[0.0; N]; N]);

        for row in 0..N {
            for col in 0..N {
                let mut value = 0.0;

                for n in 0..N {
                    value += self.0[row][n] * rhs.0[n][col]
                }

                result[row][col] = value;
            }
        }

        result
    }
}

impl<const N: usize> Mul<VectorMatrix<N>> for Matrix<N> {
    type Output = VectorMatrix<N>;

    fn mul(self, rhs: VectorMatrix<N>) -> Self::Output {
        VectorMatrix([0.0; N])
    }
}

impl<const N: usize> PartialEq for VectorMatrix<N> {
    fn eq(&self, other: &VectorMatrix<N>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let M = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(M[0][0], 1.0);
        assert_eq!(M[0][1], 2.0);
        assert_eq!(M[0][2], 3.0);
        assert_eq!(M[0][3], 4.0);

        assert_eq!(M[1][0], 5.5);
        assert_eq!(M[1][1], 6.5);
        assert_eq!(M[1][2], 7.5);
        assert_eq!(M[1][3], 8.5);

        assert_eq!(M[2][0], 9.0);
        assert_eq!(M[2][1], 10.0);
        assert_eq!(M[2][2], 11.0);
        assert_eq!(M[2][3], 12.0);

        assert_eq!(M[3][0], 13.5);
        assert_eq!(M[3][1], 14.5);
        assert_eq!(M[3][2], 15.5);
        assert_eq!(M[3][3], 16.5);
    }

    #[test]
    fn constructing_and_inspecting_a_2x2_matrix() {
        let M = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(M[0][0], -3.0);
        assert_eq!(M[0][1], 5.0);

        assert_eq!(M[1][0], 1.0);
        assert_eq!(M[1][1], -2.0);
    }

    #[test]
    fn constructing_and_inspecting_a_3x3_matrix() {
        let M = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [1.0, 1.0, 1.0]]);

        assert_eq!(M[0][0], -3.0);
        assert_eq!(M[0][1], 5.0);
        assert_eq!(M[0][2], 0.0);

        assert_eq!(M[1][0], 1.0);
        assert_eq!(M[1][1], -2.0);
        assert_eq!(M[1][2], -7.0);

        assert_eq!(M[2][0], 1.0);
        assert_eq!(M[2][1], 1.0);
        assert_eq!(M[2][2], 1.0);
    }

    #[test]
    fn matrix_values_are_mutable() {
        let mut M = Matrix::from([[1.0, 2.0], [2.0, 4.0]]);

        M[1][0] = 3.0;

        assert_eq!(M[1][0], 3.0);
    }

    #[test]
    fn comparing_matrices() {
        let A = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let B = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let C = Matrix::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        let D = Matrix::from([
            [1.0 + f64::EPSILON, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let F = Matrix::from([
            [1.0 + (2.0 * f64::EPSILON), 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(A, B);
        assert_ne!(A, C);
        assert_eq!(A, D);
        assert_ne!(A, F);
    }

    #[test]
    fn multiplying_matrices() {
        let A = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            A * B,
            Matrix::from([
                [20.0, 22.0, 50.0, 48.0],
                [44.0, 54.0, 114.0, 108.0],
                [40.0, 58.0, 110.0, 102.0],
                [16.0, 26.0, 46.0, 42.0],
            ])
        );
    }

    #[test]
    fn multiplying_matrix_and_tuple() {
        let A = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = VectorMatrix([1.0, 2.0, 3.0, 1.0]);

        assert_eq!(A * b, VectorMatrix([18.0, 24.0, 33.0, 1.0]));
    }
}
