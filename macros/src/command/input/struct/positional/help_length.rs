use super::Positional;

impl<'a> Positional<'a> {
    /// Gets the length required for the help message
    pub fn help_length(&self) -> usize {
        self.value.to_string().len()
    }
}
