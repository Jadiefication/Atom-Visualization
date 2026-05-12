use crate::complex::Complex;
use crate::vec::vec2::Vec2;
use std::f64::consts::PI;

/// Reduced Planck constant (`ħ`) in SI units.
pub const PLANCK: f64 = 1.054e-34;

/// Returns a 1D Gaussian wave packet value at position `x` with spread `sigma`.
///
/// The returned complex value is:
/// - envelope: `exp(-x² / (4σ²)) / ((2π)^(1/4) * sqrt(σ))`
/// - phase: `cos(x) + i sin(x)`
pub fn g_wave_packet(x: f64, sigma: f64) -> Complex {
    let fraction =
        (-x.powi(2) / (4.0 * sigma.powi(2))).exp() / ((2.0 * PI).powf(1.0 / 4.0) * sigma.sqrt());
    Complex::new(x.cos(), x.sin()) * fraction
}

/// Computes the discrete 2D Laplacian at `center` over a complex-valued grid.
///
/// Missing neighbors are treated as zero outside bounds.
///
/// This uses a 5-point stencil equivalent to:
/// `left + right + up + down - 4 * center`.
pub fn laplacian(center: &Vec2<usize>, grid: &Vec<Vec<Complex>>) -> Complex {
    let x = center.x;
    let y = center.y;

    let left_val = get(grid, x.saturating_sub(1), y);
    let right_val = get(grid, x + 1, y);
    let up_val = get(grid, x, y + 1);
    let down_val = get(grid, x, y.saturating_sub(1));
    let center_val = get(grid, x, y);

    left_val + right_val + up_val + down_val - center_val * 4.0
}

fn get(grid: &Vec<Vec<Complex>>, i: usize, j: usize) -> Complex {
    let max_x = grid.len();
    let max_y = if max_x > 0 { grid[0].len() } else { 0 };
    if i < max_x && j < max_y {
        grid[i][j]
    } else {
        Complex { re: 0.0, im: 0.0 }
    }
}

/// Rectified Linear Unit activation function.
///
/// Returns `num` when positive, otherwise `0.0`.
pub fn relu(num: f64) -> f64 {
    num.max(0.0)
}
