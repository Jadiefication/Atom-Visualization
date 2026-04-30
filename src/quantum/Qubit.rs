use std::ops::Index;
use crate::complex::Complex;

pub struct Qubit<const SIZE: usize> {
    pub(crate) ampls: [Complex; SIZE]
}

impl Qubit<2> {
    fn new(alpha: Complex, beta: Complex) -> Self {
        assert!((1.0 - (alpha.norm_sqr() + beta.norm_sqr())).abs() < 1e-10);
        Qubit {
            ampls: [
                alpha,
                beta
            ]
        }
    }
}

impl Default for Qubit<2> {
    fn default() -> Self {
        Self::new(Complex::new(1.0, 0.0), Complex::zero())
    }
}

impl<const SIZE: usize> Index<usize> for Qubit<SIZE> {
    type Output = Complex;
    
    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.ampls[index]
    }
}