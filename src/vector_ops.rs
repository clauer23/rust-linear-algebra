extern crate num_traits as traits;

use crate::complex;

use traits::Num;
use core::ops::Neg;

use complex::Complex;

pub fn inner_product<T: Clone + Num + Neg<Output = T>>(u: Vec<Complex<T>>, v: Vec<Complex<T>>) -> Complex<T>{
    u.iter().zip(v.iter()).map(|(x, y)| x.conj() * y.clone()).sum()
}