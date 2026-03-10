use crate::complex::Complex;
use std::f64;

const MASS: f64 = 9.11e-31;
const PLANCK: f64 = 1.054e-34;
const K_0: i8 = 1;
const PI: f64 = f64::consts::PI;
const E: f64 = f64::consts::E;

pub fn grid(start: f64, end: f64, amount: i32) -> Vec<Complex> {
    let size = end - start;
    let d_x = size / (amount as f64);
    let sigma = size / 10.0;
    let mut vec: Vec<Complex> = vec![];
    for i in 0..amount {
        let x = start + (i as f64) * d_x;
        let pos = g_wave_packet(x, sigma);
        vec.push(pos);
    }
    vec
}

pub fn g_wave_packet(x: f64, sigma: f64) -> Complex {
    let fraction = (-x.powi(2)/(4.0*sigma.powi(2))).exp() /
        ((2.0*PI).powf(1.0/4.0) * sigma.sqrt());
    Complex::new(x.cos(), x.sin()) * fraction
}