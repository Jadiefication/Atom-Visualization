use std::f32::consts::{FRAC_1_SQRT_2, PI};

use haje::complex::Complex;
use haje::matrix::Matrix;
use haje::quantum::gates::{H, I, S, SWAP, T, X, Y, Z, c_u, phase};
use haje::quantum::qubit::{Qubit, TensorQubit};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn assert_complex_eq(a: Complex, b: Complex) {
    let eps = 1e-6;
    assert!((a.re - b.re).abs() < eps, "re: {} != {}", a.re, b.re);
    assert!((a.im - b.im).abs() < eps, "im: {} != {}", a.im, b.im);
}

/// Checks size, ampls.len(), and every amplitude of a TensorQubit.
fn assert_tensor_eq(tq: &TensorQubit, expected_size: usize, expected: &[Complex]) {
    assert_eq!(
        tq.size(),
        expected_size,
        "TensorQubit.size: got {}, expected {}",
        tq.size(),
        expected_size
    );
    assert_eq!(
        expected.len(),
        expected_size,
        "test bug: expected slice has wrong length"
    );
    for i in 0..expected_size {
        assert_complex_eq(tq[i], expected[i]);
    }
}

fn zero() -> Complex {
    Complex::zero()
}

fn one() -> Complex {
    Complex::new(1.0, 0.0)
}

// ---------------------------------------------------------------------------
// Single-qubit gate value tests
// ---------------------------------------------------------------------------

#[test]
fn test_identity_gate_values() {
    assert_complex_eq(I[0][0], one());
    assert_complex_eq(I[0][1], zero());
    assert_complex_eq(I[1][0], zero());
    assert_complex_eq(I[1][1], one());
}

#[test]
fn test_pauli_x_gate_values() {
    assert_complex_eq(X[0][0], zero());
    assert_complex_eq(X[0][1], one());
    assert_complex_eq(X[1][0], one());
    assert_complex_eq(X[1][1], zero());
}

#[test]
fn test_pauli_y_gate_values() {
    assert_complex_eq(Y[0][0], zero());
    assert_complex_eq(Y[0][1], Complex::new(0.0, -1.0));
    assert_complex_eq(Y[1][0], Complex::new(0.0, 1.0));
    assert_complex_eq(Y[1][1], zero());
}

#[test]
fn test_pauli_z_gate_values() {
    assert_complex_eq(Z[0][0], one());
    assert_complex_eq(Z[0][1], zero());
    assert_complex_eq(Z[1][0], zero());
    assert_complex_eq(Z[1][1], Complex::new(-1.0, 0.0));
}

#[test]
fn test_s_gate_values() {
    assert_complex_eq(S[0][0], one());
    assert_complex_eq(S[0][1], zero());
    assert_complex_eq(S[1][0], zero());
    assert_complex_eq(S[1][1], Complex::new(0.0, 1.0));
}

#[test]
fn test_t_gate_values() {
    let v = FRAC_1_SQRT_2 as f64;
    assert_complex_eq(T[0][0], one());
    assert_complex_eq(T[0][1], zero());
    assert_complex_eq(T[1][0], zero());
    assert_complex_eq(T[1][1], Complex::new(v, v));
}

#[test]
fn test_hadamard_gate_values() {
    let v = FRAC_1_SQRT_2 as f64;
    assert_complex_eq(H[0][0], Complex::new(v, 0.0));
    assert_complex_eq(H[0][1], Complex::new(v, 0.0));
    assert_complex_eq(H[1][0], Complex::new(v, 0.0));
    assert_complex_eq(H[1][1], Complex::new(-v, 0.0));
}

// ---------------------------------------------------------------------------
// PHASE gate
// ---------------------------------------------------------------------------

#[test]
fn test_phase_gate_at_pi_over_2() {
    // P(π/2) = [[1, 0], [0, i]]
    let gate = phase(PI / 2.0);
    assert_complex_eq(gate[0][0], one());
    assert_complex_eq(gate[0][1], zero());
    assert_complex_eq(gate[1][0], zero());
    assert_complex_eq(gate[1][1], Complex::new(0.0, 1.0));
}

#[test]
fn test_phase_gate_at_pi() {
    // P(π) = [[1, 0], [0, -1]] == Z
    let gate = phase(PI);
    assert_complex_eq(gate[0][0], one());
    assert_complex_eq(gate[0][1], zero());
    assert_complex_eq(gate[1][0], zero());
    assert_complex_eq(gate[1][1], Complex::new(-1.0, 0.0));
}

#[test]
fn test_phase_gate_at_zero() {
    // P(0) = [[1, 0], [0, 1]] == I
    let gate = phase(0.0);
    assert_complex_eq(gate[0][0], one());
    assert_complex_eq(gate[0][1], zero());
    assert_complex_eq(gate[1][0], zero());
    assert_complex_eq(gate[1][1], one());
}

#[test]
fn test_phase_gate_at_pi_over_4() {
    // P(π/4) == T gate
    let v = FRAC_1_SQRT_2 as f64;
    let gate = phase(PI / 4.0);
    assert_complex_eq(gate[0][0], one());
    assert_complex_eq(gate[0][1], zero());
    assert_complex_eq(gate[1][0], zero());
    assert_complex_eq(gate[1][1], Complex::new(v, v));
}

// ---------------------------------------------------------------------------
// SWAP gate
// ---------------------------------------------------------------------------

#[test]
fn test_swap_gate_values() {
    // Row 0: [1, 0, 0, 0]
    assert_complex_eq(SWAP[0][0], one());
    assert_complex_eq(SWAP[0][1], zero());
    assert_complex_eq(SWAP[0][2], zero());
    assert_complex_eq(SWAP[0][3], zero());

    // Row 1: [0, 0, 1, 0]
    assert_complex_eq(SWAP[1][0], zero());
    assert_complex_eq(SWAP[1][1], zero());
    assert_complex_eq(SWAP[1][2], one());
    assert_complex_eq(SWAP[1][3], zero());

    // Row 2: [0, 1, 0, 0]
    assert_complex_eq(SWAP[2][0], zero());
    assert_complex_eq(SWAP[2][1], one());
    assert_complex_eq(SWAP[2][2], zero());
    assert_complex_eq(SWAP[2][3], zero());

    // Row 3: [0, 0, 0, 1]
    assert_complex_eq(SWAP[3][0], zero());
    assert_complex_eq(SWAP[3][1], zero());
    assert_complex_eq(SWAP[3][2], zero());
    assert_complex_eq(SWAP[3][3], one());
}

// ---------------------------------------------------------------------------
// Controlled-U gate
// ---------------------------------------------------------------------------

#[test]
fn test_controlled_u_gate_shape_and_values() {
    let u = Matrix {
        data: [[2.0_f32, 3.0_f32], [4.0_f32, 5.0_f32]],
    };
    let controlled: Matrix<f32, 4, 4> = c_u::<f32, 2, 4>(u);

    // Top-left 2x2 is identity
    assert_eq!(controlled[0][0], 1.0);
    assert_eq!(controlled[0][1], 0.0);
    assert_eq!(controlled[1][0], 0.0);
    assert_eq!(controlled[1][1], 1.0);

    // Bottom-right 2x2 is U
    assert_eq!(controlled[2][2], 2.0);
    assert_eq!(controlled[2][3], 3.0);
    assert_eq!(controlled[3][2], 4.0);
    assert_eq!(controlled[3][3], 5.0);

    // Off-diagonal blocks are zero
    assert_eq!(controlled[0][2], 0.0);
    assert_eq!(controlled[0][3], 0.0);
    assert_eq!(controlled[1][2], 0.0);
    assert_eq!(controlled[1][3], 0.0);
    assert_eq!(controlled[2][0], 0.0);
    assert_eq!(controlled[2][1], 0.0);
    assert_eq!(controlled[3][0], 0.0);
    assert_eq!(controlled[3][1], 0.0);
}

// ---------------------------------------------------------------------------
// Qubit basics
// ---------------------------------------------------------------------------

#[test]
fn test_qubit_default_is_zero_ket() {
    let q = Qubit::<2>::default();
    assert_complex_eq(q[0], one());
    assert_complex_eq(q[1], zero());
}

#[test]
fn test_qubit_x_gate_flips_to_one_ket() {
    let qx = Qubit::<2>::default() * X;
    assert_complex_eq(qx[0], zero());
    assert_complex_eq(qx[1], one());
}

#[test]
fn test_qubit_hadamard_produces_superposition() {
    let v = FRAC_1_SQRT_2 as f64;
    let qh = Qubit::<2>::default() * H;
    assert_complex_eq(qh[0], Complex::new(v, 0.0));
    assert_complex_eq(qh[1], Complex::new(v, 0.0));
}

#[test]
fn test_qubit_double_x_is_identity() {
    let q = Qubit::<2>::default() * X * X;
    assert_complex_eq(q[0], one());
    assert_complex_eq(q[1], zero());
}

#[test]
fn test_qubit_double_hadamard_is_identity() {
    let v = FRAC_1_SQRT_2 as f64;
    // H*H = I, so |0⟩ -> H -> H -> |0⟩
    let q = Qubit::<2>::default() * H * H;
    assert_complex_eq(q[0], one());
    assert_complex_eq(q[1], zero());
}

#[test]
fn test_qubit_z_gate_on_one_ket() {
    // Z|1⟩ = -|1⟩
    let q = Qubit::<2>::default() * X * Z;
    assert_complex_eq(q[0], zero());
    assert_complex_eq(q[1], Complex::new(-1.0, 0.0));
}

// ---------------------------------------------------------------------------
// Tensor product: Qubit & Qubit -> TensorQubit
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_zero_zero_is_00() {
    // |0⟩ ⊗ |0⟩ = |00⟩, index 0
    let pair = Qubit::<2>::default() & Qubit::<2>::default();
    assert_tensor_eq(&pair, 4, &[one(), zero(), zero(), zero()]);
}

#[test]
fn test_tensor_zero_one_is_01() {
    // |0⟩ ⊗ |1⟩ = |01⟩, index 1
    let q0 = Qubit::<2>::default();
    let q1 = Qubit::<2>::default() * X;
    let pair = q0 & q1;
    assert_tensor_eq(&pair, 4, &[zero(), one(), zero(), zero()]);
}

#[test]
fn test_tensor_one_zero_is_10() {
    // |1⟩ ⊗ |0⟩ = |10⟩, index 2
    let q0 = Qubit::<2>::default() * X;
    let q1 = Qubit::<2>::default();
    let pair = q0 & q1;
    assert_tensor_eq(&pair, 4, &[zero(), zero(), one(), zero()]);
}

#[test]
fn test_tensor_one_one_is_11() {
    // |1⟩ ⊗ |1⟩ = |11⟩, index 3
    let q0 = Qubit::<2>::default() * X;
    let q1 = Qubit::<2>::default() * X;
    let pair = q0 & q1;
    assert_tensor_eq(&pair, 4, &[zero(), zero(), zero(), one()]);
}

// ---------------------------------------------------------------------------
// Tensor product: TensorQubit & Qubit  (three qubits)
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_three_qubits_010() {
    // |0⟩ ⊗ |1⟩ ⊗ |0⟩ = |010⟩, index 2 in 8-dim space
    let partial = Qubit::<2>::default() & (Qubit::<2>::default() * X);
    assert_eq!(partial.size(), 4, "intermediate size should be 4");
    assert_eq!(partial.size(), 4);

    let triple = partial & Qubit::<2>::default();
    let mut expected = vec![zero(); 8];
    expected[2] = one();
    assert_tensor_eq(&triple, 8, &expected);
}

#[test]
fn test_tensor_three_qubits_111() {
    // |1⟩ ⊗ |1⟩ ⊗ |1⟩ = |111⟩, index 7
    let partial = (Qubit::<2>::default() * X) & (Qubit::<2>::default() * X);
    assert_eq!(partial.size(), 4);

    let triple = partial & (Qubit::<2>::default() * X);
    let mut expected = vec![zero(); 8];
    expected[7] = one();
    assert_tensor_eq(&triple, 8, &expected);
}

#[test]
fn test_tensor_three_qubits_100() {
    // |1⟩ ⊗ |0⟩ ⊗ |0⟩ = |100⟩, index 4
    let partial = (Qubit::<2>::default() * X) & Qubit::<2>::default();
    assert_eq!(partial.size(), 4);

    let triple = partial & Qubit::<2>::default();
    let mut expected = vec![zero(); 8];
    expected[4] = one();
    assert_tensor_eq(&triple, 8, &expected);
}

// ---------------------------------------------------------------------------
// Tensor product: TensorQubit & TensorQubit  (four qubits)
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_four_qubits_1111() {
    // |1⟩ ⊗ |1⟩ ⊗ |1⟩ ⊗ |1⟩ = |1111⟩, index 15 in 16-dim space
    let pair_hi = (Qubit::<2>::default() * X) & (Qubit::<2>::default() * X);
    let pair_lo = (Qubit::<2>::default() * X) & (Qubit::<2>::default() * X);
    assert_eq!(pair_hi.size(), 4);
    assert_eq!(pair_lo.size(), 4);

    let quad = pair_hi & pair_lo;
    let mut expected = vec![zero(); 16];
    expected[15] = one();
    assert_tensor_eq(&quad, 16, &expected);
}

#[test]
fn test_tensor_four_qubits_0000() {
    // |0000⟩, index 0
    let pair_hi = Qubit::<2>::default() & Qubit::<2>::default();
    let pair_lo = Qubit::<2>::default() & Qubit::<2>::default();
    let quad = pair_hi & pair_lo;

    let mut expected = vec![zero(); 16];
    expected[0] = one();
    assert_tensor_eq(&quad, 16, &expected);
}

#[test]
fn test_tensor_four_qubits_1010() {
    // |1⟩ ⊗ |0⟩ ⊗ |1⟩ ⊗ |0⟩ = |1010⟩, index 10
    let pair_hi = (Qubit::<2>::default() * X) & Qubit::<2>::default();
    let pair_lo = (Qubit::<2>::default() * X) & Qubit::<2>::default();
    let quad = pair_hi & pair_lo;

    let mut expected = vec![zero(); 16];
    expected[10] = one();
    assert_tensor_eq(&quad, 16, &expected);
}

// ---------------------------------------------------------------------------
// Tensor product: Qubit & TensorQubit  (right-hand TensorQubit)
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_qubit_and_tensor_qubit_right() {
    // |1⟩ ⊗ (|0⟩ ⊗ |1⟩) = |101⟩, index 5
    let inner = Qubit::<2>::default() & (Qubit::<2>::default() * X);
    assert_eq!(inner.size(), 4);

    let triple = (Qubit::<2>::default() * X) & inner;
    let mut expected = vec![zero(); 8];
    expected[5] = one();
    assert_tensor_eq(&triple, 8, &expected);
}

// ---------------------------------------------------------------------------
// Superposition tensor products
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_two_hadamard_uniform_superposition() {
    // H|0⟩ ⊗ H|0⟩ — all four amplitudes should be 1/2
    let pair = (Qubit::<2>::default() * H) & (Qubit::<2>::default() * H);
    let expected = vec![Complex::new(0.5, 0.0); 4];
    assert_tensor_eq(&pair, 4, &expected);
}

#[test]
fn test_tensor_three_hadamard_uniform_superposition() {
    // H|0⟩ ⊗ H|0⟩ ⊗ H|0⟩ — all eight amplitudes 1/sqrt(8)
    let make_h = || Qubit::<2>::default() * H;
    let partial = make_h() & make_h();
    assert_eq!(partial.size(), 4);

    let triple = partial & make_h();
    let amp = Complex::new(1.0 / (2.0 * (2.0_f64).sqrt()), 0.0);
    let expected = vec![amp; 8];
    assert_tensor_eq(&triple, 8, &expected);
}

#[test]
fn test_tensor_four_hadamard_uniform_superposition() {
    // H|0⟩^⊗4 — all 16 amplitudes 1/4
    let make_h = || Qubit::<2>::default() * H;
    let pair_a = make_h() & make_h();
    let pair_b = make_h() & make_h();
    let quad = pair_a & pair_b;

    let amp = Complex::new(0.25, 0.0);
    let expected = vec![amp; 16];
    assert_tensor_eq(&quad, 16, &expected);
}

// ---------------------------------------------------------------------------
// Associativity
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_associativity_left_vs_right() {
    // (|0⟩ ⊗ |1⟩) ⊗ |0⟩  ==  |0⟩ ⊗ (|1⟩ ⊗ |0⟩)
    // exercises both TensorQubit & Qubit and Qubit & TensorQubit impls
    let make_zero = || Qubit::<2>::default();
    let make_one = || Qubit::<2>::default() * X;

    let left = (make_zero() & make_one()) & make_zero();
    let right = make_zero() & (make_one() & make_zero());

    assert_eq!(left.size(), 8);
    assert_eq!(right.size(), 8);
    for i in 0..8 {
        assert_complex_eq(left[i], right[i]);
    }
}

#[test]
fn test_tensor_associativity_four_qubits() {
    // ((|1⟩ ⊗ |0⟩) ⊗ |1⟩) ⊗ |0⟩  ==  |1⟩ ⊗ (|0⟩ ⊗ (|1⟩ ⊗ |0⟩))
    // both should be |1010⟩, index 10
    let make_zero = || Qubit::<2>::default();
    let make_one = || Qubit::<2>::default() * X;

    let left = ((make_one() & make_zero()) & make_one()) & make_zero();
    let right = make_one() & (make_zero() & (make_one() & make_zero()));

    assert_eq!(left.size(), 16);
    assert_eq!(right.size(), 16);
    for i in 0..16 {
        assert_complex_eq(left[i], right[i]);
    }
}

// ---------------------------------------------------------------------------
// Norm preservation
// ---------------------------------------------------------------------------

#[test]
fn test_tensor_norm_preserved_two_qubits() {
    let pair = (Qubit::<2>::default() * H) & (Qubit::<2>::default() * X);
    assert_eq!(pair.size(), 4);

    let norm_sq: f64 = (0..pair.size())
        .map(|i| pair[i].re * pair[i].re + pair[i].im * pair[i].im)
        .sum();
    assert!((norm_sq - 1.0).abs() < 1e-6, "norm^2 = {}", norm_sq);
}

#[test]
fn test_tensor_norm_preserved_three_qubits() {
    let make_h = || Qubit::<2>::default() * H;
    let triple = (make_h() & make_h()) & make_h();
    assert_eq!(triple.size(), 8);

    let norm_sq: f64 = (0..triple.size())
        .map(|i| triple[i].re * triple[i].re + triple[i].im * triple[i].im)
        .sum();
    assert!((norm_sq - 1.0).abs() < 1e-6, "norm^2 = {}", norm_sq);
}

#[test]
fn test_tensor_norm_preserved_four_qubits() {
    let make_h = || Qubit::<2>::default() * H;
    let quad = (make_h() & make_h()) & (make_h() & make_h());
    assert_eq!(quad.size(), 16);

    let norm_sq: f64 = (0..quad.size())
        .map(|i| quad[i].re * quad[i].re + quad[i].im * quad[i].im)
        .sum();
    assert!((norm_sq - 1.0).abs() < 1e-6, "norm^2 = {}", norm_sq);
}

// ---------------------------------------------------------------------------
// into_qubit conversion
// ---------------------------------------------------------------------------

#[test]
fn test_into_qubit_correct_size_01() {
    // |0⟩ ⊗ |1⟩ -> TensorQubit(4) -> Qubit<4>
    let pair = Qubit::<2>::default() & (Qubit::<2>::default() * X);
    assert_eq!(pair.size(), 4);

    let q: Qubit<4> = pair.into_qubit::<4>();
    assert_complex_eq(q[0], zero());
    assert_complex_eq(q[1], one());
    assert_complex_eq(q[2], zero());
    assert_complex_eq(q[3], zero());
}

#[test]
fn test_into_qubit_correct_size_10() {
    // |1⟩ ⊗ |0⟩ -> Qubit<4>, index 2
    let pair = (Qubit::<2>::default() * X) & Qubit::<2>::default();
    let q: Qubit<4> = pair.into_qubit::<4>();
    assert_complex_eq(q[0], zero());
    assert_complex_eq(q[1], zero());
    assert_complex_eq(q[2], one());
    assert_complex_eq(q[3], zero());
}

#[test]
fn test_into_qubit_three_qubit_system() {
    // |0⟩ ⊗ |1⟩ ⊗ |0⟩ -> Qubit<8>, index 2
    let triple = (Qubit::<2>::default() & (Qubit::<2>::default() * X)) & Qubit::<2>::default();
    assert_eq!(triple.size(), 8);

    let q: Qubit<8> = triple.into_qubit::<8>();
    for i in 0..8usize {
        if i == 2 {
            assert_complex_eq(q[i], one());
        } else {
            assert_complex_eq(q[i], zero());
        }
    }
}

#[test]
#[should_panic(expected = "TensorQubit size 4 != Qubit<2>")]
fn test_into_qubit_wrong_size_panics() {
    let pair = Qubit::<2>::default() & Qubit::<2>::default();
    let _: Qubit<2> = pair.into_qubit::<2>();
}

#[test]
#[should_panic(expected = "TensorQubit size 4 != Qubit<8>")]
fn test_into_qubit_oversized_panics() {
    let pair = Qubit::<2>::default() & Qubit::<2>::default();
    let _: Qubit<8> = pair.into_qubit::<8>();
}
