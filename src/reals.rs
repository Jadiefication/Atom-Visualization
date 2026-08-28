use std::ops::{Add, Div, Mul, Neg, Sub};

/// Trait representing real scalar types used by generic vector operations.
///
/// This trait provides a comprehensive set of mathematical functions and constants
/// required for linear algebra, complex arithmetic, and quantum simulations.
///
/// # Examples
///
/// ```rust
/// use haje::reals::RealField;
///
/// fn compute_something<T: RealField>(x: T) -> T {
///     x.sin().powi(2) + x.cos().powi(2)
/// }
///
/// let res_f64 = compute_something(1.0f64);
/// let res_f32 = compute_something(1.0f32);
///
/// assert!((res_f64 - 1.0).abs() < 1e-10);
/// ```
///
/// Implementations are provided for `f32` and `f64`.
pub trait RealField:
    Copy
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
{
    /// Returns the additive identity `0.0`.
    fn zero() -> Self;
    /// Returns the multiplicative identity `1.0`.
    fn one() -> Self;
    /// Returns the mathematical constant PI.
    fn pi() -> Self;
    /// Returns the mathematical constant e.
    fn e() -> Self;

    /// Returns the absolute value.
    fn abs(self) -> Self;
    /// Returns a number that represents the sign of `self`.
    fn signum(self) -> Self;
    /// Returns the largest integer less than or equal to `self`.
    fn floor(self) -> Self;
    /// Returns the smallest integer greater than or equal to `self`.
    fn ceil(self) -> Self;
    /// Returns the nearest integer to `self`.
    fn round(self) -> Self;
    /// Returns the integer part of `self`.
    fn trunc(self) -> Self;
    /// Returns the fractional part of `self`.
    fn fract(self) -> Self;

    /// Returns the square root of the value.
    fn sqrt(self) -> Self;
    /// Returns the cube root of the value.
    fn cbrt(self) -> Self;
    /// Returns `e^(self)`.
    fn exp(self) -> Self;
    /// Returns `2^(self)`.
    fn exp2(self) -> Self;
    /// Returns the natural logarithm of the value.
    fn ln(self) -> Self;
    /// Returns the base 2 logarithm of the value.
    fn log2(self) -> Self;
    /// Returns the base 10 logarithm of the value.
    fn log10(self) -> Self;
    /// Returns `self` raised to an integer power.
    fn powi(self, n: i32) -> Self;
    /// Returns `self` raised to a floating point power.
    fn powf(self, n: Self) -> Self;

    /// Returns the sine of the value (in radians).
    fn sin(self) -> Self;
    /// Returns the cosine of the value (in radians).
    fn cos(self) -> Self;
    /// Returns the tangent of the value (in radians).
    fn tan(self) -> Self;
    /// Returns the arcsine of the value.
    fn asin(self) -> Self;
    /// Returns the arccosine of the value.
    fn acos(self) -> Self;
    /// Returns the arctangent of the value.
    fn atan(self) -> Self;
    /// Returns the four-quadrant arctangent.
    fn atan2(self, other: Self) -> Self;

    /// Returns the hyperbolic sine.
    fn sinh(self) -> Self;
    /// Returns the hyperbolic cosine.
    fn cosh(self) -> Self;
    /// Returns the hyperbolic tangent.
    fn tanh(self) -> Self;
    /// Returns the inverse hyperbolic sine.
    fn asinh(self) -> Self;
    /// Returns the inverse hyperbolic cosine.
    fn acosh(self) -> Self;
    /// Returns the inverse hyperbolic tangent.
    fn atanh(self) -> Self;

    /// Returns the reciprocal `1.0 / self`.
    fn recip(self) -> Self;
    /// Converts radians to degrees.
    fn to_degrees(self) -> Self;
    /// Converts degrees to radians.
    fn to_radians(self) -> Self;

    /// Returns the maximum of two values.
    fn max(self, other: Self) -> Self;
    /// Returns the minimum of two values.
    fn min(self, other: Self) -> Self;
}

macro_rules! impl_real_field {
    ($t:ident) => {
        impl RealField for $t {
            #[inline]
            fn zero() -> Self {
                0.0
            }
            #[inline]
            fn one() -> Self {
                1.0
            }
            #[inline]
            fn pi() -> Self {
                std::$t::consts::PI
            }
            #[inline]
            fn e() -> Self {
                std::$t::consts::E
            }

            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }
            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }
            #[inline]
            fn floor(self) -> Self {
                self.floor()
            }
            #[inline]
            fn ceil(self) -> Self {
                self.ceil()
            }
            #[inline]
            fn round(self) -> Self {
                self.round()
            }
            #[inline]
            fn trunc(self) -> Self {
                self.trunc()
            }
            #[inline]
            fn fract(self) -> Self {
                self.fract()
            }

            #[inline]
            fn sqrt(self) -> Self {
                self.sqrt()
            }
            #[inline]
            fn cbrt(self) -> Self {
                self.cbrt()
            }
            #[inline]
            fn exp(self) -> Self {
                self.exp()
            }
            #[inline]
            fn exp2(self) -> Self {
                self.exp2()
            }
            #[inline]
            fn ln(self) -> Self {
                self.ln()
            }
            #[inline]
            fn log2(self) -> Self {
                self.log2()
            }
            #[inline]
            fn log10(self) -> Self {
                self.log10()
            }
            #[inline]
            fn powi(self, n: i32) -> Self {
                self.powi(n)
            }
            #[inline]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            #[inline]
            fn sin(self) -> Self {
                self.sin()
            }
            #[inline]
            fn cos(self) -> Self {
                self.cos()
            }
            #[inline]
            fn tan(self) -> Self {
                self.tan()
            }
            #[inline]
            fn asin(self) -> Self {
                self.asin()
            }
            #[inline]
            fn acos(self) -> Self {
                self.acos()
            }
            #[inline]
            fn atan(self) -> Self {
                self.atan()
            }
            #[inline]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }

            #[inline]
            fn sinh(self) -> Self {
                self.sinh()
            }
            #[inline]
            fn cosh(self) -> Self {
                self.cosh()
            }
            #[inline]
            fn tanh(self) -> Self {
                self.tanh()
            }
            #[inline]
            fn asinh(self) -> Self {
                self.asinh()
            }
            #[inline]
            fn acosh(self) -> Self {
                self.acosh()
            }
            #[inline]
            fn atanh(self) -> Self {
                self.atanh()
            }

            #[inline]
            fn recip(self) -> Self {
                self.recip()
            }
            #[inline]
            fn to_degrees(self) -> Self {
                self.to_degrees()
            }
            #[inline]
            fn to_radians(self) -> Self {
                self.to_radians()
            }

            #[inline]
            fn max(self, other: Self) -> Self {
                self.max(other)
            }
            #[inline]
            fn min(self, other: Self) -> Self {
                self.min(other)
            }
        }
    };
}

impl_real_field!(f32);
impl_real_field!(f64);
