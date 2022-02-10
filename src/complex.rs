extern crate num_traits as traits;

use traits::{Zero, One, Num, Float};

use core::ops::{Add, Mul, Neg, Sub};

use std::fmt;


#[derive(PartialEq, Eq, Copy, Clone, Hash, Debug, Default)]
pub struct Complex<T>{
    // Real component
    pub re: T,
    // Imaginary Component
    pub im: T,
}

pub type Complex32 = Complex<f32>;
pub type Complex64 = Complex<f64>;

impl<T> Complex<T> {
    /// Create a new Complex
    #[inline]
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Clone + Num> Complex<T>{

    // Returns i
    pub fn i() -> Self {
        Self::new(T::zero(), T::one())
    }
}

impl<T: Clone + Num + PartialOrd + fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op = match self.im > T::zero(){
            true => "+",
            false => "",
        };
        write!(f, "{}{}{}i", self.re, op, self.im)
    }
}

impl<T: Clone + Num + Neg<Output = T>> Complex<T>{

    pub fn conj(&self) -> Self {
        Self::new(self.re.clone(), -self.im.clone())
    }
}

impl<T: Clone + Num> Add<Complex<T>> for Complex<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output{
        Self::Output::new(self.re + other.re, self.im + other.im)
    }

}

impl<T: Clone + Num> Sub<Complex<T>> for Complex<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output{
        Self::Output::new(self.re - other.re, self.im - other.im)
    }
}

impl<T: Clone + Num> Mul<Complex<T>> for Complex<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output{
        let re = self.re.clone() * other.re.clone() - self.im.clone()*other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Self::Output::new(re, im)
    }
}

impl<T: Clone + Num + Neg<Output = T>> Neg for Complex<T> {
    type Output = Self;

    fn neg(self) -> Self::Output{
        Self::Output::new(-self.re.clone(), -self.im.clone())
    }
}