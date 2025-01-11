use core::ops::{Add, Mul};

pub struct Matrix<T> {
    n: usize,
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Copy + Default + Add<Output = T> + Mul<Output = T>,
{
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![T::default(); n * n],
        }
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data
    }

    pub fn as_slice(&self) -> &[T] {
        &self.data
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        self.data[row * self.n + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.n + col] = value;
    }

    pub fn add_const(&mut self, val: T) {
        self.data.iter_mut().for_each(|x| *x = *x + val);
    }

    pub fn mul_const(&self, val: T) -> Self {
        let data = self.data.iter().map(|&x| x * val).collect();
        Self {
            n: self.n,
            data,
        }
    }

    pub fn mul_vect(&self, vector: &[T]) -> Vec<T> {
        (0..self.n)
            .map(|i| {
                (0..self.n)
                    .map(|j| self.get(i, j) * vector[j])
                    .fold(T::default(), |acc, x| acc + x)
            })
            .collect()
    }

    pub fn mul_matrix(&self, other: &Matrix<T>) -> Self {
        let mut result = Self::new(self.n);
        for i in 0..self.n {
            for j in 0..self.n {
                let sum = (0..self.n)
                    .map(|k| self.get(i, k) * other.get(k, j))
                    .fold(T::default(), |acc, x| acc + x);
                result.set(i, j, sum);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_add_const() {
        let size = 3;
        let mut matrix = Matrix::new(size);
        matrix.add_const(5);

        for i in 0..size {
            for j in 0..size {
                assert_eq!(matrix.get(i, j), 5);
            }
        }
    }

    #[test]
    fn test_matrix_mul_const() {
        let size = 2;
        let mut matrix = Matrix::new(size);
        for i in 0..size {
            for j in 0..size {
                matrix.set(i, j, (i + j) as i32);
            }
        }
        let result = matrix.mul_const(2);

        for i in 0..size {
            for j in 0..size {
                assert_eq!(result.get(i, j), (i + j) as i32 * 2);
            }
        }
    }

    #[test]
    fn test_matrix_mul_matrix() {
        let size = 2;
        let mut matrix_a = Matrix::new(size);
        let mut matrix_b = Matrix::new(size);

        matrix_a.set(0, 0, 1);
        matrix_a.set(0, 1, 2);
        matrix_a.set(1, 0, 3);
        matrix_a.set(1, 1, 4);

        matrix_b.set(0, 0, 2);
        matrix_b.set(0, 1, 0);
        matrix_b.set(1, 0, 1);
        matrix_b.set(1, 1, 2);

        let result = matrix_a.mul_matrix(&matrix_b);

        assert_eq!(result.get(0, 0), 4);
        assert_eq!(result.get(0, 1), 4);
        assert_eq!(result.get(1, 0), 10);
        assert_eq!(result.get(1, 1), 8);
    }
}
