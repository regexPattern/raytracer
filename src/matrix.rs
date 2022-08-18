use crate::utils;
use std::ops::Index;

#[derive(Debug)]
struct Matrix4x4([[f64; 4]; 4]);

impl From<[[f64; 4]; 4]> for Matrix4x4 {
    fn from(values: [[f64; 4]; 4]) -> Matrix4x4 {
        Matrix4x4(values)
    }
}

impl Index<usize> for Matrix4x4 {
    type Output = [f64; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl PartialEq for Matrix4x4 {
    fn eq(&self, other: &Matrix4x4) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if !utils::approximately_eq(self[row][col], other[row][col]) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let M = Matrix4x4::from([
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

    /* #[test]
        fn constructing_and_inspecting_a_2x2_matrix() {
            let M = Matrix4x4::from([[-3.0, 5.0], [1.0, -2.0]]);

            assert_eq!(M[0][0], -3.0);
            assert_eq!(M[0][1], 5.0);

            assert_eq!(M[1][0], 1.0);
            assert_eq!(M[1][1], -2.0);
        }

        #[test]
        fn constructing_and_inspecting_a_3x3_matrix() {
            let M = Matrix4x4::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [1.0, 1.0, 1.0]]);

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
    */
    #[test]
    fn comparing_matrices() {
        let A = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let B = Matrix4x4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let C = Matrix4x4::from([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        let D = Matrix4x4::from([
            [1.0 + f64::EPSILON, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let F = Matrix4x4::from([
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
}
