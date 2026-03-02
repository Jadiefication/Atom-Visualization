use std::ops::{Add, Sub, Mul, Div};

pub trait RealField:
Copy
+ Add<Output = Self>
+ Sub<Output = Self>
+ Mul<Output = Self>
+ Div<Output = Self>
{
    fn sqrt(self) -> Self;
}

impl RealField for f32 {
    fn sqrt(self) -> Self { f32::sqrt(self) }
}

impl RealField for f64 {
    fn sqrt(self) -> Self { f64::sqrt(self) }
}