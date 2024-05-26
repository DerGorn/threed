use std::ops::{Add, Div, Mul, Sub};

pub trait Numeric:
    Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + PartialOrd
    + PartialEq
    + Default
{
}

impl<T> Numeric for T where
    T: Copy
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + PartialOrd
        + PartialEq
        + Default
{
}
