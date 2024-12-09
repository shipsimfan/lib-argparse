use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Is this positional required
    pub fn is_required(&self) -> bool {
        T::is_required(self)
    }
}
