//! Predefined quantum gate matrices and small gate constructors.

use std::f32::consts::FRAC_1_SQRT_2;
use std::sync::LazyLock;

use crate::complex::Complex64;
use crate::matrix::Matrix;

/// Identity gate.
///
/// Leaves `|0⟩` and `|1⟩` unchanged.
pub static I: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(1.0, 0.0), Complex64::zero()],
        [Complex64::zero(), Complex64::new(1.0, 0.0)],
    ],
});

/// Pauli-X (NOT) gate.
///
/// Swaps basis states: `|0⟩ ↔ |1⟩`.
pub static X: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::zero(), Complex64::new(1.0, 0.0)],
        [Complex64::new(1.0, 0.0), Complex64::zero()],
    ],
});

/// Pauli-Y gate.
///
/// Applies a bit-flip with phase factors `±i`.
pub static Y: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::zero(), Complex64::new(0.0, -1.0)],
        [Complex64::new(0.0, 1.0), Complex64::zero()],
    ],
});

/// Pauli-Z gate.
///
/// Leaves `|0⟩` unchanged and negates the phase of `|1⟩`.
pub static Z: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(1.0, 0.0), Complex64::zero()],
        [Complex64::zero(), Complex64::new(-1.0, 0.0)],
    ],
});

/// Hadamard gate.
///
/// Creates equal superpositions from computational basis states.
pub static H: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(FRAC_1_SQRT_2 as f64, 0.0), Complex64::new(FRAC_1_SQRT_2 as f64, 0.0)],
        [Complex64::new(FRAC_1_SQRT_2 as f64, 0.0), Complex64::new(-(FRAC_1_SQRT_2 as f64), 0.0)],
    ],
});

/// Returns a phase-shift gate for `angle` radians.
///
/// This is the diagonal matrix `diag(1, e^{i*angle})`.
pub fn phase(angle: f32) -> Matrix<Complex64, 2, 2> {
    Matrix {
        data: [
            [Complex64::new(1.0, 0.0), Complex64::zero()],
            [Complex64::zero(), Complex64::new(angle.cos() as f64, angle.sin() as f64)],
        ],
    }
}

/// Two-qubit SWAP gate.
///
/// Exchanges the amplitudes of `|01⟩` and `|10⟩`.
pub static SWAP: LazyLock<Matrix<Complex64, 4, 4>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(1.0, 0.0), Complex64::zero(), Complex64::zero(), Complex64::zero()],
        [Complex64::zero(), Complex64::zero(), Complex64::new(1.0, 0.0), Complex64::zero()],
        [Complex64::zero(), Complex64::new(1.0, 0.0), Complex64::zero(), Complex64::zero()],
        [Complex64::zero(), Complex64::zero(), Complex64::zero(), Complex64::new(1.0, 0.0)],
    ],
});

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
pub static S: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(1.0, 0.0), Complex64::zero()],
        [Complex64::zero(), Complex64::new(0.0, 1.0)],
    ],
});

/// Phase gate `T`.
///
/// Equivalent to `phase(π/4)`.
pub static T: LazyLock<Matrix<Complex64, 2, 2>> = LazyLock::new(|| Matrix {
    data: [
        [Complex64::new(1.0, 0.0), Complex64::zero()],
        [Complex64::zero(), Complex64::new(FRAC_1_SQRT_2 as f64, FRAC_1_SQRT_2 as f64)],
    ],
});
