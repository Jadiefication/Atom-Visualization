# Atom-Visualization

An atom visualization project made in Rust to actually depict a real atom, without the help of external libs (except for OpenGL).

## Overview
This project aims to provide a realistic visualization of an atom using Rust. Currently, the project includes custom implementations for complex number arithmetic and vector operations (2D, 3D, and 4D), which are essential for the physics and rendering logic of an atom visualization.

## Requirements
- Rust (2024 edition)
- Cargo

## Setup
To clone and prepare the project:
```bash
git clone https://github.com/Jadiefication/Atom-Visualization.git
cd Atom
```

## Run Commands
To run the project:
```bash
cargo run
```

## Scripts
- `cargo build`: Compiles the project.
- `cargo run`: Runs the main entry point.
- `cargo test`: Runs the test suite (if any).

## Environment Variables
- TODO: Document any environment variables used for configuration (e.g., window size, simulation parameters).

## Tests
To run tests:
```bash
cargo test
```
*Note: Currently, no tests are implemented.*

## Project Structure
- `src/main.rs`: Main entry point for the application.
- `src/complex.rs`: Implementation of complex numbers and their arithmetic operations.
- `src/vec.rs`: Implementation of vector types (Vec2, Vec3, Vec4) and their operations (dot product, cross product, etc.).
- `Cargo.toml`: Project configuration and dependencies.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
