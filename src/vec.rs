use std::ops::{Add, Mul, Sub};
use crate::complex::Complex;
use crate::vec::Vec::{Vec2, Vec3, Vec4};

trait RealField: Copy + Add<Output = Self> + Mul<Output = Self> {}

// Floating-point types
impl RealField for f32 {}
impl RealField for f64 {}

// Signed integers
impl RealField for i8 {}
impl RealField for i16 {}
impl RealField for i32 {}
impl RealField for i64 {}
impl RealField for i128 {}
impl RealField for isize {}

// Unsigned integers
impl RealField for u8 {}
impl RealField for u16 {}
impl RealField for u32 {}
impl RealField for u64 {}
impl RealField for u128 {}
impl RealField for usize {}

/*
Correctly should be:

struct Vec<const N: usize, T> {
    data: [T; N]
}
*/
#[derive(Debug, Copy, Clone, PartialEq)]
enum Vec<T> {
    Vec2 { x: T, y: T },
    Vec3 { x: T, y: T, z: T },
    Vec4 { r: T, g: T, b: T, a: T }
}

trait Magnitude<T, R> {
    fn magnitude(&self) -> R;
}

trait Dot<T, R> {
    fn dot(&self, other_vec: &Vec<T>) -> R;
}

trait Cross<T> {
    fn cross(&self, other_vec: &Vec<T>) -> Vec<T>;
}

impl<T> Add<Vec<T>> for Vec<T> where T: Copy + Add<Output = T> {
    type Output = Self;

    fn add(self, rhs: Vec<T>) -> Self::Output {
        match (self, rhs) {
            (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 }) =>
                Vec2 { x: x1 + x2, y: y1 + y2 },
            (Vec3 { x: x1, y: y1, z: z1 }, Vec3 { x: x2, y: y2, z: z2 }) =>
                Vec3 { x: x1 + x2, y: y1 + y2, z: z1 + z2 },
            (Vec4 { r: r1, g: g1, b: b1, a: a1 }, Vec4 { r: r2, g: g2, b: b2, a: a2 }) => {
                Vec4 { r: r1 + r2, g: g1 + g2, b: b1 + b2, a: a1 + a2 }
            }
            _ => panic!("Mismatched vector types"),
        }
    }
}

impl<T> Sub<Vec<T>> for Vec<T> where T: Copy + Sub<Output = T> {
    type Output = Self;

    fn sub(self, rhs: Vec<T>) -> Self::Output {
        match (self, rhs) {
            (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 }) =>
                Vec2 { x: x1 - x2, y: y1 - y2 },
            (Vec3 { x: x1, y: y1, z: z1 }, Vec3 { x: x2, y: y2, z: z2 }) =>
                Vec3 { x: x1 - x2, y: y1 - y2, z: z1 - z2 },
            (Vec4 { r: r1, g: g1, b: b1, a: a1 }, Vec4 { r: r2, g: g2, b: b2, a: a2 }) => {
                Vec4 { r: r1 - r2, g: g1 - g2, b: b1 - b2, a: a1 - a2 }
            }
            _ => panic!("Mismatched vector types"),
        }
    }
}

impl<T> Mul<T> for Vec<T> where T: Copy + Mul<Output = T> {
    type Output = Vec<T>;

    fn mul(self, rhs: T) -> Self::Output {
        match self {
            Vec2 { x: x1, y: y1 } => Vec2 { x: x1 * rhs, y: y1 * rhs },
            Vec3 { x: x1, y: y1, z: z1 } => Vec3 { x: x1 * rhs, y: y1 * rhs, z: z1 * rhs },
            Vec4 { r: r1, g: g1, b: b1, a: a1 } => Vec4 { r: r1 * rhs, g: g1 * rhs, b: b1 * rhs, a: a1 * rhs },
        }
    }
}

impl Dot<Complex, f64> for Vec<Complex> {
    fn dot(&self, other_vec: &Vec<Complex>) -> f64 {
        match (self, other_vec) {
            (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 }) =>
                (x1.conj() * *x2 + y1.conj() * *y2).real,
            (Vec3 { x: x1, y: y1, z: z1 }, Vec3 { x: x2, y: y2, z: z2 }) =>
                (x1.conj() * *x2 + y1.conj() * *y2 + z1.conj() * *z2).real,
            (Vec4 { r: r1, g: g1, b: b1, a: a1 }, Vec4 { r: r2, g: g2, b: b2, a: a2 }) =>
                (r1.conj() * *r2 + g1.conj() * *g2 + b1.conj() * *b2 + a1.conj() * *a2).real,
            _ => panic!("Mismatched vector types")
        }
    }
}

impl<T> Dot<T, T> for Vec<T> where T: RealField {
    fn dot(&self, other_vec: &Vec<T>) -> T {
        match (self, other_vec) {
            (Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 }) =>
                *x1 * *x2 + *y1 * *y2,
            (Vec3 { x: x1, y: y1, z: z1 }, Vec3 { x: x2, y: y2, z: z2 }) =>
                *x1 * *x2 + *y1 * *y2 + *z1 * *z2,
            (Vec4 { r: r1, g: g1, b: b1, a: a1 }, Vec4 { r: r2, g: g2, b: b2, a: a2 }) =>
                *r1 * *r2 + *g1 * *g2 + *b1 * *b2 + *a1 * *a2,
            _ => panic!("Mismatched vector types")
        }
    }
}

impl Magnitude<Complex, f64> for Vec<Complex> {
    fn magnitude(&self) -> f64 {
        match self {
            Vec2 { x, y } => {
                (x.real * x.real + x.imaginary * x.imaginary
                    + y.real * y.real + y.imaginary * y.imaginary)
                    .sqrt()
            }
            Vec3 { x, y, z } => {
                (x.real * x.real + x.imaginary * x.imaginary
                    + y.real * y.real + y.imaginary * y.imaginary
                    + z.real * z.real + z.imaginary * z.imaginary)
                    .sqrt()
            }
            Vec4 { r, g, b, a } => {
                (r.real * r.real + r.imaginary * r.imaginary
                    + g.real * g.real + g.imaginary * g.imaginary
                    + b.real * b.real + b.imaginary * b.imaginary
                    + a.real * a.real + a.imaginary * a.imaginary)
                    .sqrt()
            }
        }
    }
}

impl<T> Cross<T> for Vec<T> where T: Copy + Sub<Output = T> + Mul<Output = T> {
    fn cross(&self, other_vec: &Vec<T>) -> Vec<T> {
        match (self, other_vec) {
            (Vec3 { x: x1, y: y1, z: z1 }, Vec3 { x: x2, y: y2, z: z2 }) => {
                Vec3 { x: *y1 * *z2 - *z1 * *y2, y: *z1 * *x2 - *x1 * *z2, z: *x1 * *y2 - *y1 * *x2 }
            },
            _ => panic!("Mismatched vector types")
        }
    }
}