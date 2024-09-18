mod matrix;
mod numeric;
mod vector;

use std::ops::Mul;

pub use matrix::Matrix;
pub use numeric::Numeric;
pub use vector::Vector;

impl<T: Numeric + Into<f32> + From<f32>> Vector<T> {
    pub fn rotate_around(&self, radians: f32, axis: &Vector<T>) -> Vector<T> {
        let matrix = Matrix::rotation(radians, axis);
        matrix * self
    }
    pub fn rotate_degree_around(&self, degree: f32, axis: &Vector<T>) -> Vector<T> {
        let matrix = Matrix::rotation_degree(degree, axis);
        matrix * self
    }
}
impl<T: Numeric + Into<f32> + From<f32>> Matrix<T> {
    pub fn rotation(radians: f32, axis: &Vector<T>) -> Self {
        let x = axis.x.into();
        let y = axis.y.into();
        let z = axis.z.into();
        let c = radians.cos();
        let s = radians.sin();
        Self::new(
            (x.powi(2) * (1.0 - c) + c).into(),
            (x * y * (1.0 - c) - z * s).into(),
            (x * z * (1.0 - c) + y * s).into(),
            (y * x * (1.0 - c) + z * s).into(),
            (y.powi(2) * (1.0 - c) + c).into(),
            (y * z * (1.0 - c) - x * s).into(),
            (z * x * (1.0 - c) - y * s).into(),
            (z * y * (1.0 - c) + x * s).into(),
            (z.powi(2) * (1.0 - c) + c).into(),
        )
    }

    pub fn rotation_degree(degree: f32, axis: &Vector<T>) -> Self {
        let radians = degree * std::f32::consts::PI / 180.0;
        Self::rotation(radians, axis)
    }
}

impl<T: Numeric> Mul<Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m12 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m13 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}
impl<T: Numeric> Mul<&Vector<T>> for Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m12 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m13 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}
impl<T: Numeric> Mul<Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m12 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m13 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}
impl<T: Numeric> Mul<&Vector<T>> for &Matrix<T> {
    type Output = Vector<T>;

    fn mul(self, rhs: &Vector<T>) -> Self::Output {
        Vector::new(
            self.m11 * rhs.x + self.m12 * rhs.y + self.m13 * rhs.z,
            self.m12 * rhs.x + self.m22 * rhs.y + self.m23 * rhs.z,
            self.m13 * rhs.x + self.m32 * rhs.y + self.m33 * rhs.z,
        )
    }
}
