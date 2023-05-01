//! Basic matrix operations using a Matrix type with internally uses
//! a vector representation to store matrix elements.
//! Generic using the [`MatrixElement`] trait, which can be implemented with
//! the [`matrix_element_type_def`] macro.
//!
//! Wikipedia reference: https://www.wikiwand.com/en/Matrix_(mathematics)
use itertools::assert_equal;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub};

/// Build a [`Matrix`] idiomatically.
#[macro_export]
macro_rules! matrix {
    [$([$($x:expr),* $(,)*]),* $(,)*] => {{
        Matrix::from(vec![$(vec![$($x,)*],)*])
    }};
}

pub trait MatrixElement:
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + AddAssign + Copy + From<u8>
{
}

/// Helper macro to implement [`MatrixElement`] for a type.
#[macro_export]
macro_rules! matrix_element_type_def {
    ($T: ty) => {
        impl MatrixElement for $T {}

        impl Mul<&Matrix<$T>> for $T {
            type Output = Matrix<$T>;

            fn mul(self, rhs: &Matrix<$T>) -> Self::Output {
                rhs * self
            }
        }
    };

    ($T: ty, $($Ti: ty),+) => {
        matrix_element_type_def!($T);
        matrix_element_type_def!($($Ti),+);
    };
}

matrix_element_type_def!(i16, i32, i64, i128, u8, u16, u32, u128, f32, f64);

/// A matrix data structure.
#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<T: MatrixElement> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: MatrixElement> Matrix<T> {
    /// Creates a new [`Matrix<T>`].
    ///
    /// # Panics
    ///
    /// Panics if the length of `data` is not equal to `rows * cols`.
    pub fn new(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert!(
            data.len() == rows * cols,
            "Inconsistent data and dimensions combination for matrix"
        );
        Self { data, rows, cols }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![0.into(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn identity(len: usize) -> Self {
        let mut identity = Matrix::zero(len, len);
        for i in 0..len {
            identity[[i, i]] = 1.into();
        }
        identity
    }

    pub fn transpose(&self) -> Self {
        let mut result = Matrix::zero(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[[i, j]] = self[[j, i]];
            }
        }
        result
    }
}

impl<T: MatrixElement> Iterator for Matrix<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.rows {
            #[allow(clippy::never_loop)]
            for j in 0..self.cols {
                return Some(self[[i, j]]);
            }
        }
        None
    }
}

impl<T: MatrixElement> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        assert!(i < self.rows && j < self.cols, "Matrix index out of bounds");
        &self.data[(self.cols * i) + j]
    }
}

impl<T: MatrixElement> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut Self::Output {
        assert!(i < self.rows && j < self.cols, "Matrix index out of bounds");
        &mut self.data[(self.cols * i) + j]
    }
}

impl<T: MatrixElement> Add<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        assert!(
            self.rows == rhs.rows && self.cols == rhs.cols,
            "Matrix dimensions do not match"
        );
        let mut result = Matrix::zero(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[[i, j]] = self[[i, j]] + rhs[[i, j]];
            }
        }
        result
    }
}

impl<T: MatrixElement> Sub for &Matrix<T> {
    type Output = Matrix<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(
            self.rows == rhs.rows && self.cols == rhs.cols,
            "Matrix dimensions do no match"
        );

        let mut result = Matrix::zero(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[[i, j]] = self[[i, j]] - rhs[[i, j]];
            }
        }
        result
    }
}

impl<T: MatrixElement> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        assert!(self.cols == rhs.rows, "Matrix dimensions do not match");

        let mut result = Matrix::zero(self.rows, rhs.cols);
        for i in 0..self.rows {
            for j in 0..rhs.cols {
                result[[i, j]] = {
                    let mut sum = 0.into();
                    for k in 0..self.cols {
                        sum += self[[i, k]] * rhs[[k, j]];
                    }
                    sum
                };
            }
        }
        result
    }
}

impl<T: MatrixElement> Mul<T> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let mut result = Matrix::zero(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[[i, j]] = rhs * self[[i, j]];
            }
        }
        result
    }
}

impl<T: MatrixElement> From<Vec<Vec<T>>> for Matrix<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let rows = v.len();
        let cols = v.first().map_or(0, |r| r.len());

        assert_equal(v.iter().map(|r| r.len()), vec![cols; rows]);
        // for row in v.iter().skip(1) {
        //     assert!(
        //         row.len() == cols,
        //         "Invalid matrix dimensions. Columns must be consistent."
        //     );
        // }
        assert!(
            !(rows != 0 && cols == 0),
            "Invalid matrix dimensions. Multiple empty rows."
        );
        Self::new(v.into_iter().flatten().collect(), rows, cols)
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::Matrix;
    use std::panic;

    const DELTA: f64 = 1e-3;

    macro_rules! assert_f64_eq {
        ($a:expr, $b:expr) => {
            assert_eq!($a.data.len(), $b.data.len());
            #[allow(trivial_numeric_casts)]
            if !$a
                .data
                .iter()
                .zip($b.data.iter())
                .all(|(x, y)| (*x as f64 - *y as f64).abs() < DELTA)
            {
                panic!();
            }
        };
    }

    #[test]
    fn invalid_matrix() {
        let result = panic::catch_unwind(|| matrix![
            [1, 0, 0, 4],
            [0, 1, 0],
            [0, 0, 1],
        ]);
        assert!(result.is_err());
    }

    #[test]
    fn empty_matrix() {
        let a: Matrix<i32> = matrix![];
        let result = panic::catch_unwind(|| a[[0, 0]]);
        assert!(result.is_err());
    }

    #[test]
    fn zero_matrix() {
        let a: Matrix<f64> = Matrix::zero(3, 5);
        let z: Matrix<f64> = matrix![
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        assert_f64_eq!(a, z);
    }

    #[test]
    fn identity_matrix() {
        let a: Matrix<f64> = Matrix::identity(5);
        let id = matrix![
            [1.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, 1.0],
        ];
        assert_f64_eq!(a, id);
    }

    #[test]
    fn invalid_add() {
        let a = matrix![
            [1, 0, 1],
            [0, 2, 0],
            [5, 0, 1]
        ];
        let err = matrix![
            [1, 2],
            [2, 4],
        ];
        let result = panic::catch_unwind(|| &a + &err);
        assert!(result.is_err());
    }

    #[test]
    fn add_i32() {
        let a = matrix![
            [1, 0, 1],
            [0, 2, 0],
            [5, 0, 1]
        ];
        let b = matrix![
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ];
        let add = matrix![
            [2, 0, 1],
            [0, 3, 0],
            [5, 0, 2],
        ];
        assert_eq!(&a + &b, add);
    }

    #[test]
    fn add_f64() {
        let a = matrix![
            [1.0, 2.0, 1.0],
            [3.0, 2.0, 0.0],
            [5.0, 0.0, 1.0],
        ];
        let b = matrix![
            [1.0, 10.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let add = matrix![
            [2.0, 12.0, 1.0],
            [3.0, 3.0, 0.0],
            [5.0, 0.0, 2.0],
        ];
        assert_f64_eq!(&a + &b, add);
    }

    #[test]
    fn invalid_sub() {
        let a = matrix![
            [2, 3],
            [10, 2],
        ];
        let err = matrix![
            [5, 6, 10],
            [7, 2, 2],
            [12, 0, 1],
        ];
        let result = panic::catch_unwind(|| &a - &err);
        assert!(result.is_err());
    }

    #[test]
    fn subtract_i32() {
        let a = matrix![
            [1, 0, 1],
            [0, 2, 0],
            [5, 0, 1],
        ];
        let b = matrix![
            [1, 0, 0],
            [0, 1, 3],
            [0, 0, 1],
        ];
        let sub = matrix![
            [0, 0, 1],
            [0, 1, -3],
            [5, 0, 0],
        ];
        assert_eq!(&a - &b, sub);
    }

    #[test]
    fn subtract_f64() {
        let a = matrix![
            [7.0, 2.0, 1.0],
            [0.0, 3.0, 2.0],
            [5.3, 8.8, std::f64::consts::PI],
        ];
        let b = matrix![
            [1.0, 0.0, 5.0],
            [-2.0, 1.0, 3.0],
            [0.0, 2.2, std::f64::consts::PI],
        ];
        let sub = matrix![
            [6.0, 2.0, -4.0],
            [2.0, 2.0, -1.0],
            [5.3, 6.6, 0.0],
        ];
        assert_f64_eq!(&a - &b, sub);
    }

    #[test]
    fn invalid_mul() {
        let a = matrix![
            [1, 2, 3],
            [4, 2, 6],
            [3, 4, 1],
            [2, 4, 8],
        ];
        let err = matrix![
            [1, 3, 3, 2],
            [7, 6, 2, 1],
            [3, 4, 2, 1],
            [3, 4, 2, 1],
        ];
        let result = panic::catch_unwind(|| &a * &err);
        assert!(result.is_err());
    }

    #[test]
    fn mul_i32() {
        let a = matrix![
            [1, 2, 3],
            [4, 2, 6],
            [3, 4, 1],
            [2, 4, 8],
        ];
        let b = matrix![
            [1, 3, 3, 2],
            [7, 6, 2, 1],
            [3, 4, 2, 1],
        ];
        let mul = matrix![
            [24, 27, 13, 7],
            [36, 48, 28, 16],
            [34, 37, 19, 11],
            [54, 62, 30, 16],
        ];
        assert_eq!(&a * &b, mul);
    }

    #[test]
    fn mul_f64() {
        let a = matrix![
            [5.5, 2.9, 1.13, 9.0],
            [0.0, 3.0, 11.0, 17.2],
            [5.3, 8.8, 2.76, 3.3],
        ];
        let b = matrix![
            [1.0, 0.3, 5.0],
            [-2.0, 1.0, 3.0],
            [-3.6, 1.5, 3.0],
            [0.0, 2.2, 2.0],
        ];
        let mul = matrix![
            [-4.368, 26.045, 57.59],
            [-45.6, 57.34, 76.4],
            [-22.236, 21.79, 67.78],
        ];
        assert_f64_eq!(&a * &b, mul);
    }

    #[test]
    fn transpose_i32() {
        let a = matrix![
            [1, 0, 1],
            [0, 2, 0],
            [5, 0, 1],
        ];
        let t = matrix![
            [1, 0, 5],
            [0, 2, 0],
            [1, 0, 1],
        ];
        assert_eq!(a.transpose(), t);
    }

    #[test]
    fn transpose_f64() {
        let a = matrix![
            [3.0, 4.0, 2.0],
            [0.0, 1.0, 3.0],
            [3.0, 1.0, 1.0],
        ];
        let t = matrix![
            [3.0, 0.0, 3.0],
            [4.0, 1.0, 1.0],
            [2.0, 3.0, 1.0],
        ];
        assert_eq!(a.transpose(), t);
    }

    #[test]
    fn matrix_scalar_zero_mul() {
        let a = matrix![
            [3, 2, 2],
            [0, 2, 0],
            [5, 4, 1],
        ];
        let scalar = 0;
        let scalar_mul = Matrix::zero(3, 3);
        assert_eq!(scalar * &a, scalar_mul);
    }

    #[test]
    fn matrix_scalar_mul_i32() {
        let a = matrix![
            [3, 2, 2],
            [0, 2, 0],
            [5, 4, 1],
        ];
        let scalar = 3;
        let scalar_mul = matrix![
            [9, 6, 6],
            [0, 6, 0],
            [15, 12, 3],
        ];
        assert_eq!(scalar * &a, scalar_mul);
    }

    #[test]
    fn matrix_scalar_mul_f64() {
        let a = matrix![
            [3.2, 5.5, 9.2],
            [1.1, 0.0, 2.3],
            [0.3, 4.2, 0.0],
        ];
        let scalar = 1.5_f64;
        let scalar_mul = matrix![
            [4.8, 8.25, 13.8],
            [1.65, 0.0, 3.45],
            [0.45, 6.3, 0.0],
        ];
        assert_f64_eq!(scalar * &a, scalar_mul);
    }
}
