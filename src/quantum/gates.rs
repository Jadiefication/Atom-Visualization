//! Predefined quantum gate matrices and small gate constructors.

use std::f32::consts::FRAC_1_SQRT_2;

use crate::complex::Complex;
use crate::matrix::Matrix;

/// Identity gate.
///
/// Leaves `|0⟩` and `|1⟩` unchanged.
pub const I: Matrix<Complex, 2, 2> = Matrix {
    data: [[Complex::new(1.0, 0.0), Complex::zero()], [Complex::zero(), Complex::new(1.0, 0.0)]],
};

/// Pauli-X (NOT) gate.
///
/// Swaps basis states: `|0⟩ ↔ |1⟩`.
pub const X: Matrix<Complex, 2, 2> = Matrix {
    data: [[Complex::zero(), Complex::new(1.0, 0.0)], [Complex::new(1.0, 0.0), Complex::zero()]],
};

/// Pauli-Y gate.
///
/// Applies a bit-flip with phase factors `±i`.
pub const Y: Matrix<Complex, 2, 2> = Matrix {
    data: [[Complex::zero(), Complex::new(0.0, -1.0)], [Complex::new(0.0, 1.0), Complex::zero()]],
};

/// Pauli-Z gate.
///
/// Leaves `|0⟩` unchanged and negates the phase of `|1⟩`.
pub const Z: Matrix<Complex, 2, 2> = Matrix {
    data: [[Complex::new(1.0, 0.0), Complex::zero()], [Complex::zero(), Complex::new(-1.0, 0.0)]],
};

/// Hadamard gate.
///
/// Creates equal superpositions from computational basis states.
pub const H: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(FRAC_1_SQRT_2 as f64, 0.0), Complex::new(FRAC_1_SQRT_2 as f64, 0.0)],
        [Complex::new(FRAC_1_SQRT_2 as f64, 0.0), Complex::new(-(FRAC_1_SQRT_2 as f64), 0.0)],
    ],
};

/// Returns a phase-shift gate for `angle` radians.
///
/// This is the diagonal matrix `diag(1, e^{i*angle})`.
pub fn phase(angle: f32) -> Matrix<Complex, 2, 2> {
    Matrix {
        data: [
            [Complex::new(1.0, 0.0), Complex::zero()],
            [Complex::zero(), Complex::new(angle.cos() as f64, angle.sin() as f64)],
        ],
    }
}

/// Two-qubit SWAP gate.
///
/// Exchanges the amplitudes of `|01⟩` and `|10⟩`.
pub const SWAP: Matrix<Complex, 4, 4> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero(), Complex::zero(), Complex::zero()],
        [Complex::zero(), Complex::zero(), Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(1.0, 0.0), Complex::zero(), Complex::zero()],
        [Complex::zero(), Complex::zero(), Complex::zero(), Complex::new(1.0, 0.0)],
    ],
};

/// Builds a controlled-`U` matrix from a gate `u`.
///
/// Produces a block matrix of shape `(2N) x (2N)` equivalent to:
/// `[[I_N, 0], [0, U]]`.
pub fn c_u<T, const N: usize, const DOUBLE_N: usize>(
    u: Matrix<T, N, N>,
) -> Matrix<T, DOUBLE_N, DOUBLE_N>
where
    T: Default + Copy + From<f32>,
{
    Matrix::from_fn(|i, j| {
        if i < N && j < N {
            if i == j { T::from(1.0) } else { T::from(0.0) }
        } else if i >= N && j >= N {
            u.data[i - N][j - N]
        } else {
            T::from(0.0)
        }
    })
}

/// Phase gate `S`.
///
/// Equivalent to `phase(π/2)`.
pub const S: Matrix<Complex, 2, 2> = Matrix {
    data: [[Complex::new(1.0, 0.0), Complex::zero()], [Complex::zero(), Complex::new(0.0, 1.0)]],
};

/// Phase gate `T`.
///
/// Equivalent to `phase(π/4)`.
pub const T: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(FRAC_1_SQRT_2 as f64, FRAC_1_SQRT_2 as f64)],
    ],
};
