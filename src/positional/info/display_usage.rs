use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Displays the usage value for this positional on stdout
    pub fn display_usage(&self) {
        let required = T::is_required(self);

        if !required {
            print!("[");
        }

        print!("{}", self.value);

        if T::multiple(self) {
            print!("..");
        }

        if !required {
            print!("]");
        }

        print!(" ");
    }
}
