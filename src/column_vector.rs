extern crate num_traits as traits;

use std::ops::{Deref, DerefMut};

#[derive(PartialEq, Eq, Clone, Hash, Debug, Default)]
pub struct ColumnV<T>(Vec<T>);

impl<T> Deref for ColumnV<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for ColumnV<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
