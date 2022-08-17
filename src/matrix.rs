use std::ops::{Index, Mul};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Index2D {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Matrix {
    values: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(rows: Vec<Vec<f64>>) -> Matrix {
        Matrix { values: rows }
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        self.values
            .iter()
            .flatten()
            .zip(other.values.iter().flatten())
            .all(|(a, b)| (a - b).abs() < (2.0 * f64::EPSILON))
    }
}

impl Index<i32> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: i32) -> &Self::Output {
        &self.values[index as usize]
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        let values = Vec::new();


        Matrix::new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_2_by_2_matrix() {
        let A = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);

        assert_eq!(A[0][0], -3.0);
        assert_eq!(A[0][1], 5.0);

        assert_eq!(A[1][0], 1.0);
        assert_eq!(A[1][1], -2.0);
    }

    #[test]
    fn creating_3_by_3_matrix() {
        let A = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);

        assert_eq!(A[0][0], -3.0);
        assert_eq!(A[0][1], 5.0);
        assert_eq!(A[0][2], 0.0);

        assert_eq!(A[1][0], 1.0);
        assert_eq!(A[1][1], -2.0);
        assert_eq!(A[1][2], -7.0);

        assert_eq!(A[2][0], 0.0);
        assert_eq!(A[2][1], 1.0);
        assert_eq!(A[2][2], 1.0);
    }

    #[test]
    fn creating_4_by_4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][1], 2.0);
        assert_eq!(m[0][2], 3.0);
        assert_eq!(m[0][3], 4.0);

        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][1], 6.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[1][3], 8.5);

        assert_eq!(m[2][0], 9.0);
        assert_eq!(m[2][1], 10.0);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[2][3], 12.0);

        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][1], 14.5);
        assert_eq!(m[3][2], 15.5);
        assert_eq!(m[3][3], 16.5);
    }

    #[test]
    fn comparing_two_matrices() {
        let A = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let C = Matrix::new(vec![
            vec![2.0, 2.0, 3.0, 4.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);

        assert_eq!(A, B);
        assert_ne!(A, C);

        let D = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0 + f64::EPSILON],
        ]);

        let E = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0 + (2.0 * f64::EPSILON)],
        ]);

        assert_eq!(A, D);
        assert_ne!(A, E);
    }

    #[test]
    fn multiplying_two_matrices() {
        let A = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);

        let B = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);

        assert_eq!(
            A * B,
            Matrix::new(vec![
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ])
        );
    }
}
