use alloc::vec::Vec;
use core::ops::{Add, Div, Mul, Sub};

pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd + From<i8>,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![T::default(); rows * cols],
        }
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.cols + col] = value;
    }

    pub fn add_const(&mut self, val: T) {
        self.data.iter_mut().for_each(|x| *x = *x + val);
    }

    pub fn mul_const(&self, val: T) -> Self {
        let data = self.data.iter().map(|&x| x * val).collect();
        Self {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }

    pub fn mul_vect(&self, vector: &[T]) -> Vec<T> {
        (0..self.rows)
            .map(|i| {
                (0..self.cols)
                    .map(|j| self.get(i, j) * vector[j])
                    .fold(T::default(), |acc, x| acc + x)
            })
            .collect()
    }

    pub fn mul_matrix(&self, other: &Matrix<T>) -> Self {
        let mut result = Self::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let sum = (0..self.cols)
                    .map(|k| self.get(i, k) * other.get(k, j))
                    .fold(T::default(), |acc, x| acc + x);
                result.set(i, j, sum);
            }
        }
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        let n = self.rows;
        let mut augmented = Matrix::new(n, 2 * n);

        for i in 0..n {
            for j in 0..n {
                augmented.set(i, j, self.get(i, j));
            }
            augmented.set(i, n + i, T::from(1));
        }

        for i in 0..n {
            let mut pivot_row = i;
            for j in (i + 1)..n {
                if augmented.get(j, i) > augmented.get(pivot_row, i) {
                    pivot_row = j;
                }
            }
            if augmented.get(pivot_row, i) == T::default() {
                return None;
            }
            if pivot_row != i {
                for j in 0..2 * n {
                    let temp = augmented.get(i, j);
                    augmented.set(i, j, augmented.get(pivot_row, j));
                    augmented.set(pivot_row, j, temp);
                }
            }

            let pivot = augmented.get(i, i);
            for j in 0..2 * n {
                augmented.set(i, j, augmented.get(i, j) / pivot);
            }

            for k in 0..n {
                if k != i {
                    let factor = augmented.get(k, i);
                    for j in 0..2 * n {
                        let value = augmented.get(k, j) - factor * augmented.get(i, j);
                        augmented.set(k, j, value);
                    }
                }
            }
        }

        let mut inverse = Matrix::new(n, n);
        for i in 0..n {
            for j in 0..n {
                inverse.set(i, j, augmented.get(i, n + j));
            }
        }
        Some(inverse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_add_const() {
        let (rows, cols) = (3, 4);
        let mut matrix = Matrix::new(rows, cols);
        matrix.add_const(5);

        for i in 0..rows {
            for j in 0..cols {
                assert_eq!(matrix.get(i, j), 5);
            }
        }
    }

    #[test]
    fn test_matrix_mul_const() {
        let (rows, cols) = (3, 4);
        let mut matrix = Matrix::new(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                matrix.set(i, j, (i * cols + j) as i32);
            }
        }

        let result = matrix.mul_const(2);

        for i in 0..rows {
            for j in 0..cols {
                assert_eq!(result.get(i, j), (i * cols + j) as i32 * 2);
            }
        }
    }

    #[test]
    fn test_matrix_mul_vect() {
        let (rows, cols) = (2, 2);
        let mut matrix = Matrix::new(rows, cols);

        matrix.set(0, 0, 1);
        matrix.set(0, 1, 2);
        matrix.set(1, 0, 3);
        matrix.set(1, 1, 4);

        let vector = vec![2, 1];
        let result = matrix.mul_vect(&vector);

        assert_eq!(result, vec![4, 10]);
    }

    #[test]
    fn test_square_mul_matrix() {
        let size = 3;
        let mut matrix_a = Matrix::new(size, size);
        let mut matrix_b = Matrix::new(size, size);

        for i in 0..size {
            for j in 0..size {
                matrix_a.set(i, j, (i + 1) as i32 * (j + 1) as i32);
                matrix_b.set(i, j, (j + 1) as i32 * (i + 1) as i32);
            }
        }

        let result = matrix_a.mul_matrix(&matrix_b);

        for i in 0..size {
            for j in 0..size {
                let expected = (0..size).map(|k| matrix_a.get(i, k) * matrix_b.get(k, j)).sum::<i32>();
                assert_eq!(result.get(i, j), expected);
            }
        }
    }

    #[test]
    fn test_non_square_mul_matrix() {
        let (a, b) = (2, 3);
        let mut matrix_a = Matrix::new(a, b);
        let mut matrix_b = Matrix::new(b, a);

        for i in 0..a {
            for j in 0..b {
                matrix_a.set(i, j, i as i32 + j as i32);
            }
        }
        for i in 0..b {
            for j in 0..a {
                matrix_b.set(i, j, i as i32 - j as i32);
            }
        }

        let result = matrix_a.mul_matrix(&matrix_b);

        for i in 0..a {
            for j in 0..a {
                let expected = (0..b).map(|k| matrix_a.get(i, k) * matrix_b.get(k, j)).sum::<i32>();
                assert_eq!(result.get(i, j), expected);
            }
        }
    }

    #[test]
    fn test_edge_cases_mul_matrix() {
        // Empty matrix (0 x 3) * (3 x 2)
        let matrix_a = Matrix::<i32>::new(0, 3);
        let matrix_b = Matrix::<i32>::new(3, 2);
        let result = matrix_a.mul_matrix(&matrix_b);
        assert_eq!(result.rows, 0);
        assert_eq!(result.cols, 2);

        // Single row (1 x 3) * (3 x 1)
        let mut matrix_c = Matrix::new(1, 3);
        for j in 0..3 {
            matrix_c.set(0, j, j as i32 + 1);
        }
        let mut matrix_d = Matrix::new(3, 1);
        for i in 0..3 {
            matrix_d.set(i, 0, i as i32 + 1);
        }
        let result = matrix_c.mul_matrix(&matrix_d);
        assert_eq!(result.rows, 1);
        assert_eq!(result.cols, 1);
        assert_eq!(result.get(0, 0), 14); // 1*1 + 2*2 + 3*3

        // Single col (3 x 1) * (1 x 3)
        let result = matrix_d.mul_matrix(&matrix_c);
        assert_eq!(result.rows, 3);
        assert_eq!(result.cols, 3);
        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(result.get(i, j), (i as i32 + 1) * (j as i32 + 1));
            }
        }
    }

    #[test]
    fn test_inverse() {
        let size = 2;
        let mut matrix = Matrix::new(size, size);

        matrix.set(0, 0, 4.0);
        matrix.set(0, 1, 7.0);
        matrix.set(1, 0, 2.0);
        matrix.set(1, 1, 6.0);

        let inverse = matrix.inverse();
        assert!(inverse.is_some());

        let inverse = inverse.unwrap();

        let identity = matrix.mul_matrix(&inverse);

        for i in 0..size {
            for j in 0..size {
                if i == j {
                    assert!((identity.get(i, j) as f32 - 1.0).abs() < 1e-6);
                } else {
                    assert!((identity.get(i, j) as f32).abs() < 1e-6);
                }
            }
        }
    }
}
