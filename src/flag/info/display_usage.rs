use crate::{flag::DEFUALT_FLAG_VALUE, Flag, FlagInfo};

impl<T: Flag> FlagInfo<T> {
    /// Displays the usage value for this flag on stdout, returning true if this flag is optional and won't display
    pub fn display_usage(&self) -> bool {
        if !T::is_required(self) {
            return true;
        }

        if let Some(short_name) = self.short_name {
            print!("{}", short_name);
            if self.long_name.is_some() {
                print!("/");
            }
        }

        if let Some(long_name) = self.long_name {
            print!("{}", long_name);
        }

        if let Some(value) = self.value {
            print!(" {}", value);
        } else if T::takes_value(self) {
            print!(" {}", DEFUALT_FLAG_VALUE);
        }

        print!(" ");

        false
    }
}
