use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Are multiple values allowed
    pub fn multiple(&self) -> bool {
        T::multiple(self)
    }
}
