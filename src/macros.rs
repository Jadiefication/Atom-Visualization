macro_rules! impl_binary_ops {
    (Complex; $( $trait:ident :: $fn:ident => $op:tt ),*) => {
        $(
            impl<T: Real> $trait for Complex<T> {
                type Output = Self;

                fn $fn(self, other: Self) -> Self {
                    Self {
                        re: self.re $op other.re,
                        im: self.im $op other.im,
                    }
                }
            }
        )*
    };

    (CScalar; $( $trait:ident :: $fn:ident => $op:tt ),*) => {
        $(
            impl<T: Real> $trait<T> for Complex<T> {
                type Output = Self;

                fn $fn(self, other: T) -> Self {
                    Self {
                        re: self.re $op other,
                        im: self.im $op other,
                    }
                }
            }
        )*
    };

    (Matrix; $( $trait:ident :: $fn:ident => $op:tt ),*) => {
        $(impl<T, const ROWS: usize, const COLS: usize> $trait for Matrix<T, ROWS, COLS>
        where
            for<'a> &'a T: $trait<Output = T>,
        {
            type Output = Matrix<T, ROWS, COLS>;

            fn $fn(self, rhs: Self) -> Self::Output {
                let left = self.data;
                let right = rhs.data;
                Matrix::from_fn(|i, j| &left[i][j] $op &right[i][j])
            }
        })*
    };

    (MScalar; $( $trait:ident :: $fn:ident => $op:tt ),*) => {
        $(impl<T, const ROWS: usize, const COLS: usize> $trait<T> for Matrix<T, ROWS, COLS>
        where
            for<'a> &'a T: $trait<&'a T, Output = T>,
        {
            type Output = Matrix<T, ROWS, COLS>;

            fn $fn(self, rhs: T) -> Self::Output {
                let left = self.data;
                Matrix::from_fn(|i, j| &left[i][j] $op &rhs)
            }
        })*
    };
}
