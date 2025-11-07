use crate::flag_group::output::InProgress;

impl<'a> InProgress<'a> {
    /// Gets the number of contained types
    pub fn len(&self) -> usize {
        self.types.len()
    }
}
