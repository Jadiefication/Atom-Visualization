use crate::reals::RealField;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec4<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T> Vec4<T>
where
    T: RealField,
{
    pub fn dot(self, other: Self) -> T {
        self.r * other.r
            + self.g * other.g
            + self.b * other.b
            + self.a * other.a
    }

    pub fn magnitude(self) -> T {
        (self.r * self.r
            + self.g * self.g
            + self.b * self.b
            + self.a * self.a)
            .sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.magnitude()
    }
}

impl<T> Add for Vec4<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a
        }
    }
}

impl<T> Sub for Vec4<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a
        }
    }
}

impl<T> Mul<T> for Vec4<T>
where
    T: Copy + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs
        }
    }
}

impl<T> Div<T> for Vec4<T>
where
    T: Copy + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
            a: self.a / rhs
        }
    }
}