use std::{
    f64::consts::TAU,
    ops::{Add, Mul, MulAssign, Sub},
};

#[derive(Debug, Clone, Copy)]
pub struct Complex64 {
    pub re: f64,
    pub im: f64,
}

impl Default for Complex64 {
    #[inline]
    fn default() -> Self {
        Self { re: 0.0, im: 0.0 }
    }
}

impl Complex64 {
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline]
    pub fn square_norm(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    #[inline]
    pub fn norm(&self) -> f64 {
        self.square_norm().sqrt()
    }

    #[inline]
    pub fn inverse(&self) -> Self {
        let norm = self.square_norm();
        Self {
            re: self.re / norm,
            im: -self.im / norm,
        }
    }
}

impl Add<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn add(self, rhs: Complex64) -> Self::Output {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl Sub<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn sub(self, rhs: Complex64) -> Self::Output {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl Mul<Complex64> for Complex64 {
    type Output = Complex64;

    #[inline]
    fn mul(self, rhs: Complex64) -> Self::Output {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl MulAssign<Complex64> for Complex64 {
    #[inline]
    fn mul_assign(&mut self, rhs: Complex64) {
        // *self = *self * rhs;
        let tmp = self.re * rhs.im + self.im * rhs.re;
        self.re = self.re * rhs.re - self.im * rhs.im;
        self.im = tmp;
    }
}

pub fn fast_fourier_transform_input_permutation(length: usize) -> Vec<usize> {
    let mut result = Vec::new();
    result.reserve_exact(length);
    // let mut result = Vec::with_capacity(length);
    for i in 0..length {
        result.push(i);
    }

    let mut reverse = 0usize;
    let mut position = 1usize;

    while position < length {
        let mut bit = length >> 1;

        while bit & reverse != 0 {
            reverse ^= bit;
            bit >>= 1;
        }
        reverse ^= bit;

        // This is equivalent to adding 1 to a reversed number
        if position < reverse {
            result.swap(position, reverse);
        }

        position += 1;
    }

    result
}

pub fn fast_fourier_transform(input: &[f64], input_permutation: &[usize]) -> Vec<Complex64> {
    let n = input.len();
    let mut result = Vec::new();
    result.reserve_exact(n);
    // let mut result = Vec::with_capacity(n);
    for position in input_permutation {
        result.push(Complex64::new(input[*position], 0.0));
    }

    let mut seg_len = 1usize;
    while seg_len < n {
        seg_len <<= 1;
        let angle = TAU / seg_len as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());

        for seg_start in (0..n).step_by(seg_len) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in seg_start..(seg_start + seg_len / 2) {
                let a = result[position];
                let b = result[position + seg_len / 2] * w;
                result[position] = a + b;
                result[position + seg_len / 2] = a - b;
                w *= w_len;
            }
        }
    }

    result
}

pub fn inverse_fast_fourier_transform(
    input: &[Complex64],
    input_permutation: &[usize],
) -> Vec<f64> {
    let n = input.len();
    // let mut result = Vec::with_capacity(n);
    let mut result = Vec::new();
    result.reserve_exact(n);
    for position in input_permutation {
        result.push(input[*position]);
    }

    let mut seg_len = 1usize;
    while seg_len < n {
        seg_len <<= 1;
        let angle = TAU / seg_len as f64;
        let w_len = Complex64::new(angle.cos(), angle.sin());

        for seg_start in (0..n).step_by(seg_len) {
            let mut w = Complex64::new(1.0, 0.0);
            for position in seg_start..(seg_start + seg_len / 2) {
                let a = result[position];
                let b = result[position + seg_len / 2] * w;
                result[position] = a + b;
                result[position + seg_len / 2] = a - b;
                w *= w_len;
            }
        }
    }

    let scale = 1.0 / n as f64;
    result.iter().map(|x| x.re * scale).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    fn almost_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // const EPSILON: f64 = 1e-6;

    #[test]
    fn small_polynomial_returns_self() {
        let polynomial = vec![1.0f64, 1.0, 0.0, 2.5];
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let fft = fast_fourier_transform(&polynomial, &permutation);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        for (x, y) in ifft.iter().zip(polynomial.iter()) {
            assert!(almost_equal(*x, *y));
        }
    }

    #[test]
    fn square_small_polynomial() {
        let mut polynomial = vec![1.0f64, 1.0, 0.0, 2.0];
        polynomial.append(&mut vec![0.0; 4]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = vec![1.0, 2.0, 1.0, 4.0, 4.0, 0.0, 4.0, 0.0, 0.0];
        for (x, y) in ifft.iter().zip(expected.iter()) {
            assert!(almost_equal(*x, *y));
        }
    }

    #[test]
    #[ignore]
    fn square_big_polynomial() {
        // This test case takes ~1050ms on my machine in unoptimized mode,
        // but it takes ~70ms in release mode.
        let n = 1 << 17; // ~100_000
        let mut polynomial = vec![1.0f64; n];
        polynomial.append(&mut vec![0.0f64; n]);
        let permutation = fast_fourier_transform_input_permutation(polynomial.len());
        let mut fft = fast_fourier_transform(&polynomial, &permutation);
        fft.iter_mut().for_each(|num| *num *= *num);
        let ifft = inverse_fast_fourier_transform(&fft, &permutation);
        let expected = (0..((n << 1) - 1)).map(|i| std::cmp::min(i + 1, (n << 1) - 1 - i) as f64);
        for (&x, y) in ifft.iter().zip(expected) {
            assert!(almost_equal(x, y));
        }
    }
}
