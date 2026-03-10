use Atom::grid::{grid, g_wave_packet};
use std::f64;

#[test]
fn test_g_wave_packet_zero() {
    let sigma = 1.0;
    let x = 0.0;
    let res = g_wave_packet(x, sigma);
    
    // x = 0.0
    // fraction = exp(-0 / (4 * 1)) / ((2 * PI)^0.25 * sqrt(1))
    // fraction = 1.0 / (2 * PI)^0.25
    let expected_fraction = 1.0 / (2.0 * f64::consts::PI).powf(0.25);
    
    // Complex::new(cos(0), sin(0)) * fraction = (1.0, 0.0) * fraction
    let expected_real = expected_fraction;
    let expected_imaginary = 0.0;
    
    assert!((res.real - expected_real).abs() < 1e-10);
    assert!((res.imaginary - expected_imaginary).abs() < 1e-10);
}

#[test]
fn test_g_wave_packet_non_zero() {
    let sigma = 2.0;
    let x = 1.0;
    let res = g_wave_packet(x, sigma);
    
    let expected_fraction = (-x.powi(2) / (4.0 * sigma.powi(2))).exp() /
        ((2.0 * f64::consts::PI).powf(0.25) * sigma.sqrt());
    
    let expected_real = x.cos() * expected_fraction;
    let expected_imaginary = x.sin() * expected_fraction;
    
    assert!((res.real - expected_real).abs() < 1e-10);
    assert!((res.imaginary - expected_imaginary).abs() < 1e-10);
}

#[test]
fn test_grid_length() {
    let start = 0.0;
    let end = 10.0;
    let amount = 50;
    let res = grid(start, end, amount);
    
    assert_eq!(res.len(), amount as usize);
}

#[test]
fn test_grid_values() {
    let start = -5.0;
    let end = 5.0;
    let amount = 10;
    let res = grid(start, end, amount);
    
    let size = end - start;
    let d_x = size / (amount as f64);
    let sigma = size / 10.0;
    
    for i in 0..amount {
        let x = start + (i as f64) * d_x;
        let expected = g_wave_packet(x, sigma);
        
        assert!((res[i as usize].real - expected.real).abs() < 1e-10);
        assert!((res[i as usize].imaginary - expected.imaginary).abs() < 1e-10);
    }
}
