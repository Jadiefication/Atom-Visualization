//! Complex-number support used across the crate.
//!
//! The [`Complex`](crate::complex::Complex) type is intentionally small and `Copy`, making it suitable
//! for numeric code where values are passed by value frequently (for example,
//! matrix and qubit operations).

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::real::Real;

/// A complex number represented as `re + i * im`.
///
/// The implementation uses `f64` components and supports arithmetic with both
/// complex and real (`f64`) operands.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex<T: Real> {
    /// Real part.
    pub re: T,
    /// Imaginary part.
    pub im: T,
}

pub type Complex64 = Complex<f64>;

impl<T: Real> Complex<T> {
    /// Creates a new complex number from real and imaginary parts.
    ///
    /// # Example
    /// ```rust
    /// use haje::complex::Complex;
    ///
    /// let z = Complex::new(3.0, -2.0);
    /// assert_eq!(z.re, 3.0);
    /// assert_eq!(z.im, -2.0);
    /// ```
    pub const fn new(real: T, imaginary: T) -> Self {
        Self { re: real, im: imaginary }
    }

    /// Returns the complex conjugate `re - i * im`.
    pub fn conj(&self) -> Self {
        Complex { re: self.re, im: -self.im }
    }

    /// Returns the magnitude `|z| = sqrt(re² + im²)`.
    pub fn mag(&self) -> T {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    /// Returns the phase angle (argument) in radians.
    ///
    /// Internally this uses `atan2(im, re)`.
    pub fn arg(&self) -> T {
        self.im.atan2(self.re)
    }

    /// Returns the complex exponential `e^z`.
    ///
    /// For `z = a + ib`, this computes `e^a (cos(b) + i sin(b))`.
    pub fn exp(&self) -> Self {
        let exp_real = self.re.exp();

        Self { re: exp_real * self.im.cos(), im: exp_real * self.im.sin() }
    }

    /// Returns the squared magnitude `|z|² = re² + im²`.
    ///
    /// This avoids the square root required by [`Complex::mag`].
    pub fn norm_sqr(&self) -> T {
        self.re.powi(2) + self.im.powi(2)
    }

    /// Returns the additive identity `0 + 0i`.
    pub fn zero() -> Self {
        Self { re: T::zero(), im: T::zero() }
    }

    pub fn one_im() -> Self {
        Self { re: T::zero(), im: T::one() }
    }
}

impl<T: Real> Add for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { re: self.re + other.re, im: self.im + other.im }
    }
}

impl<T: Real> Sub for Complex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { re: self.re - other.re, im: self.im - other.im }
    }
}

impl<T: Real> Mul for Complex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl<T: Real> Div for Complex<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.re.powi(2) + other.im.powi(2);

        Self {
            re: (self.re * other.re + self.im * other.im) / denom,
            im: (self.im * other.re - self.re * other.im) / denom,
        }
    }
}

impl<T: Real> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self { re: -self.re, im: -self.im }
    }
}

impl<T: Real> Add<T> for Complex<T> {
    type Output = Self;

    fn add(self, other: T) -> Self {
        Self { re: self.re + other, im: self.im }
    }
}

impl<T: Real> Sub<T> for Complex<T> {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Self { re: self.re - other, im: self.im }
    }
}

impl<T: Real> Mul<T> for Complex<T> {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Self { re: self.re * other, im: self.im * other }
    }
}

impl<T: Real> Div<T> for Complex<T> {
    type Output = Self;

    fn div(self, other: T) -> Self {
        Self { re: self.re / other, im: self.im / other }
    }
}

impl<T: Real> Rem<T> for Complex<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self { re: self.re % rhs, im: self.im % rhs }
    }
}

impl<T: Real> From<T> for Complex<T> {
    fn from(val: T) -> Self {
        Complex::new(val, T::zero())
    }
}
