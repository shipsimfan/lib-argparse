use crate::{Positional, PositionalInfo};

impl<T: Positional> PositionalInfo<T> {
    /// Displays the help for this positional on stdout
    pub fn display_help(&self, description_offset: usize) {
        print!("  {}", self.value);

        if let Some(description) = self.description {
            for _ in 0..description_offset - self.value.len() - 2 {
                print!(" ");
            }

            description(description_offset);
        }

        println!();
    }
}
