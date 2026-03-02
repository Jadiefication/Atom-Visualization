use std::ops::{Add, Div, Mul, Sub};
use crate::reals::RealField;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: RealField,
{
    pub fn dot(&self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(&self) -> T {
        (self.x * self.x + self.y * self.y).sqrt()
    }

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