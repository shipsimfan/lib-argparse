use crate::{Flag, FlagInfo};

impl<T: Flag> FlagInfo<T> {
    /// Displays the help for this flag on stdout
    pub fn display_help(&self, short_names: bool, description_offset: usize) {
        let mut offset = 3;
        if let Some(short_name) = self.short_name {
            print!("{}", short_name);

            if self.long_name.is_some() {
                print!(",");
            }
        } else if short_names {
            print!("   ");
        } else {
            offset = 0;
        }

        if let Some(long_name) = self.long_name {
            print!("{}", long_name);
            offset += long_name.len();
        }

        if let Some(value) = self.value {
            print!(" {}", value);
            offset += value.len() + 1;
        } else if T::takes_value(self) {
            print!(" VALUE");
            offset += 6;
        }

        if let Some(description) = self.description {
            for _ in 0..description_offset - offset {
                print!(" ");
            }

            description(description_offset);

            if let Some(default) = self.default {
                print!(" [Default: {}]", default().as_display());
            }
        }

        println!();
    }
}
