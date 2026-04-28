use crate::complex::Complex;
use crate::matrix::Matrix;

pub const I: Matrix<i8, 2, 2> = Matrix {
    data: [
        [1, 0],
        [0, 1]
    ]
};

pub const X: Matrix<i8, 2, 2> = Matrix {
    data: [
        [0, 1],
        [1, 0]
    ]
};

pub const Y: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::zero(), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::zero()]
    ]
};

pub const Z: Matrix<i8, 2, 2> = Matrix {
    data: [
        [1, 0],
        [0, -1]
    ]
};