# Project Improvements

This document tracks planned and suggested improvements for the Haje library.

## Core Math & Types

### 1. Generic Complex Numbers
- **Goal**: Make `Complex` generic over `T: RealField`.
- **Reason**: Allows use of `f32` or other custom numeric types for lower precision or specific hardware requirements.
- **Tasks**:
  - Refactor `src/complex.rs` to use generics.
  - Implement `From<T>` for `Complex<T>`.

### 2. Standard Trait Derivations
- **Goal**: Derive common Rust traits for all core types (`Matrix`, `Qubit`, `TensorQubit`).
- **Reason**: Improves usability (e.g., printing with `{:?}`, comparing with `==`, cloning).
- **Traits**: `Debug`, `Clone`, `Copy` (where possible), `PartialEq`, `Eq`.

### 3. Integration with `num-traits`
- **Goal**: Implement traits from the `num-traits` crate.
- **Reason**: Standardizes the numeric API and allows interoperability with other Rust math libraries.

### 4. Advanced Matrix Operations
- **Goal**: Add LU decomposition, Inverse, and Eigenvalue calculations.
- **Reason**: Essential for many linear algebra applications beyond basic arithmetic.

## Quantum Computing

### 1. Measurement Logic
- **Goal**: Add methods to `Qubit` and `TensorQubit` for probabilistic measurement.
- **Reason**: Enables simulation of quantum algorithms that require observing the final state.

### 2. Expanded Gate Library
- **Goal**: Add more predefined gates like `Toffoli`, `Fredkin`, and rotation gates (Rx, Ry, Rz).
- **Reason**: Provides a richer set of building blocks for quantum circuits.

### 3. State Vector Normalization
- **Goal**: Automatically verify or enforce normalization of qubit states.
- **Reason**: Prevents invalid quantum states from being processed.

## Performance & Infrastructure

### 1. GPU Acceleration Completion
- **Goal**: Fully implement the optional `gpu` feature using `wgpu`.
- **Reason**: Offloads heavy computations (especially tensor products) to the GPU.
- **Tasks**:
  - Fix and expand WGSL shaders in `shaders/`.
  - Implement the `Gpu` singleton and buffer management in `src/priv_gpu/gpu.rs`.

### 2. SIMD Optimizations
- **Goal**: Use SIMD instructions for vector and matrix operations.
- **Reason**: Significantly boosts performance on CPUs for large batches of operations.

### 3. Documentation & Examples
- **Goal**: Expand `rustdoc` comments and add a dedicated `examples/` directory.
- **Reason**: Makes the library easier to learn and use for newcomers.
