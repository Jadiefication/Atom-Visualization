use std::ops::{Add, Div, Mul, Sub};

/// Trait representing real scalar types used by generic vector operations.
///
/// This trait is intentionally small and captures only the capabilities needed by
/// this crate's generic vector methods (`dot`, `magnitude`, `normalize`).
///
/// Implementations are provided for `f32` and `f64`.
pub trait RealField:
    Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>
{
    /// Returns the square root of the value.
    fn sqrt(self) -> Self;
}

impl RealField for f32 {
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }
}

impl RealField for f64 {
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }
}
