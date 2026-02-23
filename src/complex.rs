use std::arch::aarch64::vrsqrted_f64;
use std::ops;

pub struct Complex {
    real: f64,
    imaginary: f64
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
        self.real.atan2(self.imaginary)
    }

    pub fn exp(&self) -> Self {
        Complex { real: self.real.exp() * self.imaginary.cos(),
            imaginary: self.real.exp() * self.imaginary.sin() }
    }
}