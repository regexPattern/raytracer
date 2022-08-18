use crate::utils;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug)]
struct Matrix<const R: usize, const C: usize>([[f64; C]; R]);

impl<const R: usize, const C: usize> Index<usize> for Matrix<R, C> {
    type Output = [f64; C];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const R: usize, const C: usize> IndexMut<usize> for Matrix<R, C> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const R: usize, const C: usize> PartialEq for Matrix<R, C> {
    fn eq(&self, other: &Matrix<R, C>) -> bool {
        for row in 0..R {
            for col in 0..C {
                if !utils::approximately_eq(self.0[row][col], other.0[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const R1: usize, const C1: usize, const C2: usize> Mul<Matrix<C1, C2>> for Matrix<R1, C1> {
    type Output = Matrix<R1, C2>;

    fn mul(self, rhs: Matrix<C1, C2>) -> Self::Output {
        let mut result = Matrix([[0.0; C2]; R1]);

        for row1 in 0..R1 {
            for col2 in 0..C2 {
                let mut value = 0.0;
                for col1 in 0..C1 {
                    value += self.0[row1][col1] * rhs.0[col1][col2]
                }
                result[row1][col2] = value;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
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
    fn constructing_and_inspecting_an_MxN_matrix() {
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
            [1.0 + f64::EPSILON, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m3 = Matrix([
            [1.0 + (2.0 * f64::EPSILON), 2.0, 3.0, 4.0],
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
        let m2 = Matrix([[1.0 + f64::EPSILON], [2.0], [3.0]]);
        let m3 = Matrix([[1.0 + (2.0 * f64::EPSILON)], [2.0], [3.0]]);

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
        let A = Matrix([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let b = Matrix([[1.0], [2.0], [3.0], [1.0]]);

        // assert_eq!(A * b, Matrix([[18.0], [24.0], [33.0], [1.0]]));
    }
}
