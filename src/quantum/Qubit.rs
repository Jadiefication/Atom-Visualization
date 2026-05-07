use std::ops::{BitAnd, Index, Mul};
use crate::complex::Complex;
use crate::matrix::Matrix;

pub struct Qubit<const SIZE: usize> {
    pub(crate) ampls: [Complex; SIZE]
}

pub struct TensorQubit {
    pub ampls: Vec<Complex>,
    pub size: usize
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

impl Index<usize> for TensorQubit {
    type Output = Complex;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.ampls[index]
    }
}

impl<const SIZE: usize> Mul<Matrix<Complex, SIZE, SIZE>> for Qubit<SIZE> {
    type Output = Qubit<SIZE>;

    fn mul(self, matrix: Matrix<Complex, SIZE, SIZE>) -> Self::Output {
        let mut new_qubit = Qubit { ampls: [Complex::zero(); SIZE] };
        for (i, vec) in matrix.data.iter().enumerate() {
            for (j, complex) in vec.iter().enumerate() {
                new_qubit.ampls[i] = new_qubit[i] + (*complex * self.ampls[j])
            }
        }
        new_qubit
    }
}

impl<const L: usize, const R: usize> BitAnd<Qubit<R>> for Qubit<L> {
    type Output = TensorQubit;

    fn bitand(self, rhs: Qubit<R>) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); L * R];

        for i in 0..L {
            for j in 0..R {
                new_ampls[i * R + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls, size: L * R }
    }
}

impl BitAnd<TensorQubit> for TensorQubit {
    type Output = TensorQubit;

    fn bitand(self, rhs: TensorQubit) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); self.size * rhs.size];

        for i in 0..self.size {
            for j in 0..rhs.size {
                new_ampls[i * rhs.size + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls, size: self.size * rhs.size }
    }
}

impl<const R: usize> BitAnd<Qubit<R>> for TensorQubit {
    type Output = TensorQubit;

    fn bitand(self, rhs: Qubit<R>) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); self.size * R];

        for i in 0..self.size {
            for j in 0..R {
                new_ampls[i * R + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls, size: self.size * R }
    }
}

impl<const L: usize> BitAnd<TensorQubit> for Qubit<L> {
    type Output = TensorQubit;

    fn bitand(self, rhs: TensorQubit) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); L * rhs.size];

        for i in 0..L {
            for j in 0..rhs.size {
                new_ampls[i * rhs.size + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls, size: L * rhs.size }
    }
}

impl TensorQubit {
    pub fn into_qubit<const N: usize>(self) -> Qubit<N> {
        assert_eq!(self.size, N, "TensorQubit size {} != Qubit<{}>", self.size, N);
        let mut ampls = [Complex::zero(); N];
        ampls.copy_from_slice(&self.ampls);
        Qubit { ampls }
    }
}
