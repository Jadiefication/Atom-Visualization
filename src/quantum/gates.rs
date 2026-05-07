use std::f32::consts::FRAC_1_SQRT_2;
use crate::complex::Complex;
use crate::matrix::Matrix;

pub const I: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(1.0, 0.0)]
    ]
};

pub const X: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::zero(), Complex::new(1.0, 0.0)],
        [Complex::new(1.0, 0.0), Complex::zero()]
    ]
};

pub const Y: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::zero(), Complex::new(0.0, -1.0)],
        [Complex::new(0.0, 1.0), Complex::zero()]
    ]
};

pub const Z: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(-1.0, 0.0)]
    ]
};

pub const H: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(FRAC_1_SQRT_2 as f64, 0.0), Complex::new(FRAC_1_SQRT_2 as f64, 0.0)],
        [Complex::new(FRAC_1_SQRT_2 as f64, 0.0), Complex::new(-(FRAC_1_SQRT_2 as f64), 0.0)]
    ]
};

pub fn phase(angle: f32) -> Matrix<Complex, 2, 2> {
    Matrix {
        data: [
            [Complex::new(1.0, 0.0), Complex::zero()],
            [Complex::zero(), Complex::new(angle.cos() as f64, angle.sin() as f64)]
        ]
    }
}

pub const SWAP: Matrix<Complex, 4, 4> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero(), Complex::zero(), Complex::zero()],
        [Complex::zero(), Complex::zero(), Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(1.0, 0.0), Complex::zero(), Complex::zero()],
        [Complex::zero(), Complex::zero(), Complex::zero(), Complex::new(1.0, 0.0)]
    ]
};

pub fn c_u<T, const N: usize, const DOUBLE_N: usize>(u: Matrix<T, N, N>) -> Matrix<T, DOUBLE_N, DOUBLE_N>
where
    T: Default + Copy + From<f32>
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

pub const S: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(0.0, 1.0)]
    ]
};

pub const T: Matrix<Complex, 2, 2> = Matrix {
    data: [
        [Complex::new(1.0, 0.0), Complex::zero()],
        [Complex::zero(), Complex::new(FRAC_1_SQRT_2 as f64, FRAC_1_SQRT_2 as f64)]
    ]
};
