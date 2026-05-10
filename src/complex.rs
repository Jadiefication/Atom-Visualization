//! Complex-number support used across the crate.
//!
//! The [`Complex`] type is intentionally small and `Copy`, making it suitable
//! for numeric code where values are passed by value frequently (for example,
//! matrix and qubit operations).

use std::ops::{Add, Div, Mul, Neg, Sub};

/// A complex number represented as `re + i * im`.
///
/// The implementation uses `f64` components and supports arithmetic with both
/// complex and real (`f64`) operands.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    /// Real part.
    pub re: f64,
    /// Imaginary part.
    pub im: f64
}

impl Complex {
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
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { re: real, im: imaginary }
    }

    /// Returns the additive identity `0 + 0i`.
    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    /// Returns the complex conjugate `re - i * im`.
    pub fn conj(&self) -> Self {
        Complex { re: self.re, im: self.im * -1.0 }
    }

    /// Returns the magnitude `|z| = sqrt(re² + im²)`.
    pub fn mag(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    /// Returns the phase angle (argument) in radians.
    ///
    /// Internally this uses `atan2(im, re)`.
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Returns the complex exponential `e^z`.
    ///
    /// For `z = a + ib`, this computes `e^a (cos(b) + i sin(b))`.
    pub fn exp(&self) -> Self {
        let exp_real = self.re.exp();

        Self {
            re: exp_real * self.im.cos(),
            im: exp_real * self.im.sin(),
        }
    }

    /// Returns the squared magnitude `|z|² = re² + im²`.
    ///
    /// This avoids the square root required by [`Complex::mag`].
    pub fn norm_sqr(&self) -> f64 {
        self.re.powi(2) + self.im.powi(2)
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im
                + self.im * other.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.re.powi(2) + other.im.powi(2);

        Self {
            re: (self.re * other.re + self.im * other.im) / denom,
            im: (self.im * other.re - self.re * other.im) / denom,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            re: self.re + other,
            im: self.im,
        }
    }
}

impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            re: self.re - other,
            im: self.im,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            re: self.re * other,
            im: self.im * other
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            re: self.re / other,
            im: self.im / other
        }
    }
}