//! Qubit state containers and tensor-product composition helpers.

use std::ops::{BitAnd, Index, Mul};

use crate::complex::Complex;
use crate::matrix::Matrix;

/// A fixed-size qubit register represented by complex amplitudes.
///
/// `SIZE` is the number of amplitudes in the state vector. For a full register
/// of `n` qubits, this is typically `2^n`.
pub struct Qubit<const SIZE: usize> {
    pub(crate) ampls: [Complex<f64>; SIZE],
}

/// A dynamically-sized tensor-product qubit state.
///
/// This is used when the state size is not known at compile time, especially
/// after repeated tensor-product operations.
pub struct TensorQubit {
    pub(crate) ampls: Vec<Complex<f64>>,
}

impl Qubit<2> {
    fn new(alpha: Complex<f64>, beta: Complex<f64>) -> Self {
        assert!((1.0 - (alpha.norm_sqr() + beta.norm_sqr())).abs() < 1e-10);
        Qubit { ampls: [alpha, beta] }
    }
}

impl Default for Qubit<2> {
    /// Returns the computational basis state `|0⟩`.
    fn default() -> Self {
        Self::new(Complex::new(1.0, 0.0), Complex::zero())
    }
}

impl<const SIZE: usize> Index<usize> for Qubit<SIZE> {
    type Output = Complex<f64>;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.ampls[index]
    }
}

impl Index<usize> for TensorQubit {
    type Output = Complex<f64>;

    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        &self.ampls[index]
    }
}

impl<const SIZE: usize> Mul<Matrix<Complex<f64>, SIZE, SIZE>> for Qubit<SIZE> {
    type Output = Qubit<SIZE>;

    fn mul(self, matrix: Matrix<Complex<f64>, SIZE, SIZE>) -> Self::Output {
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
        TensorQubit { ampls: new_ampls }
    }
}

impl BitAnd<TensorQubit> for TensorQubit {
    type Output = TensorQubit;

    fn bitand(self, rhs: TensorQubit) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); self.ampls.len() * rhs.ampls.len()];

        for i in 0..self.ampls.len() {
            for j in 0..rhs.ampls.len() {
                new_ampls[i * rhs.ampls.len() + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls }
    }
}

impl<const R: usize> BitAnd<Qubit<R>> for TensorQubit {
    type Output = TensorQubit;

    fn bitand(self, rhs: Qubit<R>) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); self.ampls.len() * R];

        for i in 0..self.ampls.len() {
            for j in 0..R {
                new_ampls[i * R + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls }
    }
}

impl<const L: usize> BitAnd<TensorQubit> for Qubit<L> {
    type Output = TensorQubit;

    fn bitand(self, rhs: TensorQubit) -> Self::Output {
        let mut new_ampls = vec![Complex::zero(); L * rhs.ampls.len()];

        for i in 0..L {
            for j in 0..rhs.ampls.len() {
                new_ampls[i * rhs.ampls.len() + j] = self.ampls[i] * rhs.ampls[j];
            }
        }
        TensorQubit { ampls: new_ampls }
    }
}

impl TensorQubit {
    /// Converts this tensor state into a fixed-size [`Qubit`] of size `N`.
    ///
    /// Panics if the number of amplitudes does not match `N`.
    pub fn into_qubit<const N: usize>(self) -> Qubit<N> {
        assert_eq!(self.ampls.len(), N, "TensorQubit size {} != Qubit<{}>", self.ampls.len(), N);
        let mut ampls = [Complex::zero(); N];
        ampls.copy_from_slice(&self.ampls);
        Qubit { ampls }
    }

    /// Returns the number of amplitudes in the tensor state.
    ///
    /// For a normalized `n`-qubit state, this is usually `2^n`.
    pub fn size(&self) -> usize {
        self.ampls.len()
    }
}
