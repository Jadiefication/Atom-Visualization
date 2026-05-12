use crate::reals::RealField;
use std::ops::{Add, Div, Mul, Sub};

/// A two-dimensional vector.
///
/// `Vec2<T>` is generic over scalar type `T`, typically `f32` or `f64`.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    /// X component.
    pub x: T,
    /// Y component.
    pub y: T,
}

impl<T> Vec2<T>
where
    T: RealField,
{
    /// Returns the dot product of `self` and `other`.
    ///
    /// In coordinates: `x1*x2 + y1*y2`.
    pub fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Returns the Euclidean length of the vector.
    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a normalized vector with unit magnitude.
    ///
    /// This divides each component by [`Vec2::magnitude`].
    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl<T> Add for Vec2<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
