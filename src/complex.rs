use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64
}

impl Complex {
    pub fn new() -> Self { Complex { real: 0.0, imaginary: 0.0 } }

    pub fn add(&self, other: Complex) -> Self {
        Complex { real: self.real + other.real, imaginary: self.imaginary + other.imaginary }
    }

    pub fn sub(&self, other: Complex) -> Self {
        Complex { real: self.real - other.real, imaginary: self.imaginary - other.imaginary }
    }

    pub fn mul(&self, other: Complex) -> Self {
        Complex { real: (self.real * other.real) - (self.imaginary * other.imaginary),
            imaginary: (self.real * other.imaginary) + (self.imaginary * other.real) }
    }

    pub fn conj(&self) -> Self {
        Complex { real: self.real, imaginary: self.imaginary * -1.0 }
    }

    pub fn mag(&self) -> f64 {
        (self.real.powi(2) + self.imaginary.powi(2)).sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.imaginary.atan2(self.real)
    }

    pub fn exp(&self) -> Self {
        Complex { real: self.real.exp() * self.imaginary.cos(),
            imaginary: self.real.exp() * self.imaginary.sin() }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary
                + self.imaginary * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let denom = other.real.powi(2) + other.imaginary.powi(2);

        Self {
            real: (self.real * other.real + self.imaginary * other.imaginary) / denom,
            imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denom,
        }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Add<f64> for Complex {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            real: self.real + other,
            imaginary: self.imaginary,
        }
    }
}

impl Sub<f64> for Complex {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            real: self.real - other,
            imaginary: self.imaginary,
        }
    }
}

impl Mul<f64> for Complex {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            real: self.real * other,
            imaginary: self.imaginary * other
        }
    }
}

impl Div<f64> for Complex {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            real: self.real / other,
            imaginary: self.imaginary / other
        }
    }
}