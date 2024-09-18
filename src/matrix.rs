use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::numeric::Numeric;

pub struct Matrix<T: Numeric> {
    pub m11: T,
    pub m12: T,
    pub m13: T,
    pub m21: T,
    pub m22: T,
    pub m23: T,
    pub m31: T,
    pub m32: T,
    pub m33: T,
}
impl<T: Numeric> Default for Matrix<T> {
    fn default() -> Self {
        Self {
            m11: T::default(),
            m12: T::default(),
            m13: T::default(),
            m21: T::default(),
            m22: T::default(),
            m23: T::default(),
            m31: T::default(),
            m32: T::default(),
            m33: T::default(),
        }
    }
}
impl<T: Numeric> Matrix<T> {
    pub fn new(m11: T, m12: T, m13: T, m21: T, m22: T, m23: T, m31: T, m32: T, m33: T) -> Self {
        Self {
            m11,
            m12,
            m13,
            m21,
            m22,
            m23,
            m31,
            m32,
            m33,
        }
    }

    pub fn scalar(scalar: T) -> Self {
        Self {
            m11: scalar,
            m12: scalar,
            m13: scalar,
            m21: scalar,
            m22: scalar,
            m23: scalar,
            m31: scalar,
            m32: scalar,
            m33: scalar,
        }
    }

    pub fn determinant(&self) -> T {
        self.m11 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m12 * (self.m21 * self.m33 - self.m23 * self.m31)
            + self.m13 * (self.m21 * self.m32 - self.m22 * self.m31)
    }

    pub fn transpose(&self) -> Self {
        Self {
            m11: self.m11,
            m12: self.m21,
            m13: self.m31,
            m21: self.m12,
            m22: self.m22,
            m23: self.m32,
            m31: self.m13,
            m32: self.m23,
            m33: self.m33,
        }
    }
}
impl<T: Numeric + Into<f32> + From<f32>> Matrix<T> {
    pub fn unity() -> Self {
        Self {
            m11: T::from(1.0),
            m12: T::from(0.0),
            m13: T::from(0.0),
            m21: T::from(0.0),
            m22: T::from(1.0),
            m23: T::from(0.0),
            m31: T::from(0.0),
            m32: T::from(0.0),
            m33: T::from(1.0),
        }
    }
    pub fn inverse(&self) -> Option<Matrix<T>> {
        let det = self.determinant().into();
        if det == 0.0 {
            return None;
        }
        let inv_det = (1.0 / det).into();
        Some(Self {
            m11: (self.m22 * self.m33 - self.m23 * self.m32) * inv_det,
            m12: (self.m13 * self.m32 - self.m12 * self.m33) * inv_det,
            m13: (self.m12 * self.m23 - self.m13 * self.m22) * inv_det,
            m21: (self.m23 * self.m31 - self.m21 * self.m33) * inv_det,
            m22: (self.m11 * self.m33 - self.m13 * self.m31) * inv_det,
            m23: (self.m21 * self.m13 - self.m11 * self.m23) * inv_det,
            m31: (self.m21 * self.m32 - self.m22 * self.m31) * inv_det,
            m32: (self.m12 * self.m31 - self.m11 * self.m32) * inv_det,
            m33: (self.m11 * self.m22 - self.m12 * self.m21) * inv_det,
        })
    }
}

impl<T: Numeric> Mul for Matrix<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}
impl<T: Numeric> Mul<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}
impl<T: Numeric> Mul for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}
impl<T: Numeric> Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        Matrix {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
        }
    }
}
impl<T: Numeric> MulAssign for Matrix<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = &*self * rhs;
    }
}
impl<T: Numeric> MulAssign<&Matrix<T>> for Matrix<T> {
    fn mul_assign(&mut self, rhs: &Matrix<T>) {
        *self = &*self * rhs;
    }
}

macro_rules! impl_matric_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op for Matrix<T> {
            type Output = Self;

            fn $method(self, rhs: Self) -> Self::Output {
                Self {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m13: self.m13.$method(rhs.m13),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                    m23: self.m23.$method(rhs.m23),
                    m31: self.m31.$method(rhs.m31),
                    m32: self.m32.$method(rhs.m32),
                    m33: self.m33.$method(rhs.m33),
                }
            }
        }
        impl<T: Numeric> $op<&Matrix<T>> for Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: &Self) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m13: self.m13.$method(rhs.m13),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                    m23: self.m23.$method(rhs.m23),
                    m31: self.m31.$method(rhs.m31),
                    m32: self.m32.$method(rhs.m32),
                    m33: self.m33.$method(rhs.m33),
                }
            }
        }
        impl<T: Numeric> $op for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: Self) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m13: self.m13.$method(rhs.m13),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                    m23: self.m23.$method(rhs.m23),
                    m31: self.m31.$method(rhs.m31),
                    m32: self.m32.$method(rhs.m32),
                    m33: self.m33.$method(rhs.m33),
                }
            }
        }
        impl<T: Numeric> $op<Matrix<T>> for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: Matrix<T>) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs.m11),
                    m12: self.m12.$method(rhs.m12),
                    m13: self.m13.$method(rhs.m13),
                    m21: self.m21.$method(rhs.m21),
                    m22: self.m22.$method(rhs.m22),
                    m23: self.m23.$method(rhs.m23),
                    m31: self.m31.$method(rhs.m31),
                    m32: self.m32.$method(rhs.m32),
                    m33: self.m33.$method(rhs.m33),
                }
            }
        }
        impl<T: Numeric> $op_assign for Matrix<T> {
            fn $method_assign(&mut self, rhs: Self) {
                self.m11 = self.m11.$method(rhs.m11);
                self.m12 = self.m12.$method(rhs.m12);
                self.m13 = self.m13.$method(rhs.m13);
                self.m21 = self.m21.$method(rhs.m21);
                self.m22 = self.m22.$method(rhs.m22);
                self.m23 = self.m23.$method(rhs.m23);
                self.m31 = self.m31.$method(rhs.m31);
                self.m32 = self.m32.$method(rhs.m32);
                self.m33 = self.m33.$method(rhs.m33);
            }
        }
        impl<T: Numeric> $op_assign<&Matrix<T>> for Matrix<T> {
            fn $method_assign(&mut self, rhs: &Self) {
                self.m11 = self.m11.$method(rhs.m11);
                self.m12 = self.m12.$method(rhs.m12);
                self.m13 = self.m13.$method(rhs.m13);
                self.m21 = self.m21.$method(rhs.m21);
                self.m22 = self.m22.$method(rhs.m22);
                self.m23 = self.m23.$method(rhs.m23);
                self.m31 = self.m31.$method(rhs.m31);
                self.m32 = self.m32.$method(rhs.m32);
                self.m33 = self.m33.$method(rhs.m33);
            }
        }
    };
    () => {};
}
impl_matric_op!(Add, AddAssign, add, add_assign);
impl_matric_op!(Sub, SubAssign, sub, sub_assign);

macro_rules! impl_scalar_op {
    ($op:ident, $op_assign:ident, $method:ident, $method_assign:ident) => {
        impl<T: Numeric> $op<T> for Matrix<T> {
            type Output = Self;

            fn $method(self, rhs: T) -> Self::Output {
                Self {
                    m11: self.m11.$method(rhs),
                    m12: self.m12.$method(rhs),
                    m13: self.m13.$method(rhs),
                    m21: self.m21.$method(rhs),
                    m22: self.m22.$method(rhs),
                    m23: self.m23.$method(rhs),
                    m31: self.m31.$method(rhs),
                    m32: self.m32.$method(rhs),
                    m33: self.m33.$method(rhs),
                }
            }
        }
        impl<T: Numeric> $op<T> for &Matrix<T> {
            type Output = Matrix<T>;

            fn $method(self, rhs: T) -> Self::Output {
                Matrix {
                    m11: self.m11.$method(rhs),
                    m12: self.m12.$method(rhs),
                    m13: self.m13.$method(rhs),
                    m21: self.m21.$method(rhs),
                    m22: self.m22.$method(rhs),
                    m23: self.m23.$method(rhs),
                    m31: self.m31.$method(rhs),
                    m32: self.m32.$method(rhs),
                    m33: self.m33.$method(rhs),
                }
            }
        }
        impl<T: Numeric> $op_assign<T> for Matrix<T> {
            fn $method_assign(&mut self, rhs: T) {
                self.m11 = self.m11.$method(rhs);
                self.m12 = self.m12.$method(rhs);
                self.m13 = self.m13.$method(rhs);
                self.m21 = self.m21.$method(rhs);
                self.m22 = self.m22.$method(rhs);
                self.m23 = self.m23.$method(rhs);
                self.m31 = self.m31.$method(rhs);
                self.m32 = self.m32.$method(rhs);
                self.m33 = self.m33.$method(rhs);
            }
        }
    };
    () => {};
}

impl_scalar_op!(Mul, MulAssign, mul, mul_assign);
impl_scalar_op!(Div, DivAssign, div, div_assign);

