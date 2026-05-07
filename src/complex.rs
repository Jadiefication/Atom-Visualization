use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64
}

impl Complex {
    pub const fn new(real: f64, imaginary: f64) -> Self {
        Self { re: real, im: imaginary }
    }

    pub const fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn conj(&self) -> Self {
        Complex { re: self.re, im: self.im * -1.0 }
    }

    pub fn mag(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).sqrt()
    }

    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn exp(&self) -> Self {
        let exp_real = self.re.exp();

        Self {
            re: exp_real * self.im.cos(),
            im: exp_real * self.im.sin(),
        }
    }

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