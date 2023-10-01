use crate::{Error, FlagArgument};
use std::ops::Deref;

/// Represents a set of flag arguments
///
/// No two flags in this structure can share either a short or long name
pub(crate) struct FlagSet<T, E: 'static>(Vec<FlagArgument<T, E>>);

impl<T, E> FlagSet<T, E> {
    /// Creates a new empty [`FlagSet`]
    pub(crate) fn new() -> Self {
        FlagSet(Vec::new())
    }

    /// Generates the help display
    ///
    ///  - `f` is the output
    pub(crate) fn generate_help(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        short_prefix: &str,
        long_prefix: &str,
    ) -> std::fmt::Result {
        if self.len() == 0 {
            return Ok(());
        }

        writeln!(f)?;
        writeln!(f, "OPTIONS:")?;

        let mut longest_short_name = 0;
        let mut longest_total = 0;
        for argument in self {
            let mut total_length = 0;
            if let Some(short_name) = argument.short_name() {
                let mut short_name_len = short_name.len() + short_prefix.len();
                if argument.long_name().is_some() {
                    short_name_len += 2;
                }

                if short_name_len > longest_short_name {
                    longest_short_name = short_name_len;
                }

                total_length += short_name_len;
            }

            if let Some(long_name) = argument.long_name() {
                total_length += long_name.len() + long_prefix.len();
            }

            if total_length > longest_total {
                longest_total = total_length;
            }
        }

        let short_padding = longest_short_name;
        let total_padding = longest_total + 2;

        /*
         * -s, --long HINT  Description
         */
        for argument in self {
            write!(f, "  ")?;

            let mut length = 0;
            match argument.short_name() {
                Some(short_name) => {
                    write!(f, "{}{}", short_prefix, short_name)?;
                    length += short_prefix.len() + short_name.len();
                }
                None => {
                    for _ in 0..short_padding {
                        write!(f, " ")?;
                    }
                    length += short_padding;
                }
            }

            match argument.long_name() {
                Some(long_name) => {
                    if argument.short_name().is_some() {
                        write!(f, ", ")?;
                        length += 2;
                    }

                    write!(f, "{}{}", long_prefix, long_name)?;
                    length += long_prefix.len() + long_name.len();
                }
                None => {}
            }

            for _ in 0..total_padding - length {
                write!(f, " ")?;
            }

            writeln!(f, "{}", argument.description())?;
        }

        Ok(())
    }

    /// Gets an argument based on its short name
    ///
    ///  - `short_name` is the short name that will be searched for
    pub(crate) fn get_short(&mut self, short_name: &str) -> Option<&mut FlagArgument<T, E>> {
        for flag in &mut self.0 {
            if let Some(f_short_name) = flag.short_name() {
                if f_short_name == short_name {
                    return Some(flag);
                }
            }
        }

        None
    }

    /// Gets an argument based on its long name
    ///
    ///  - `long_name` is the long name that will be searched for
    pub(crate) fn get_long(&mut self, long_name: &str) -> Option<&mut FlagArgument<T, E>> {
        for flag in &mut self.0 {
            if let Some(f_long_name) = flag.long_name() {
                if f_long_name == long_name {
                    return Some(flag);
                }
            }
        }

        None
    }

    /// Pushes an argument into the set
    ///
    /// Returns any arguments which have conflicting names.
    /// If there is only one conflict, the conflicting argument will always be the first value of the returned tuple.
    ///
    ///  - `flag_argument` is the argument to be pushed into the set
    pub(crate) fn push(
        &mut self,
        flag_argument: FlagArgument<T, E>,
    ) -> (Option<FlagArgument<T, E>>, Option<FlagArgument<T, E>>) {
        assert!(
            flag_argument.short_name().is_some() || flag_argument.long_name().is_some(),
            "FlagArgument must have at least a short name or a long name"
        );

        let ret = self.remove(flag_argument.short_name(), flag_argument.long_name());
        self.0.push(flag_argument);
        ret
    }

    /// Finalizes all the arguments in this set
    pub(crate) fn finalize(&mut self) -> Result<(), Error<E>> {
        for flag_argument in self {
            flag_argument.finalize()?;
        }

        Ok(())
    }

    /// Removes arguments based on their long or short names
    ///
    /// Returns any arguments which have conflicting names.
    /// If there is only one conflict, the conflicting argument will always be the first value of the returned tuple.
    ///
    ///  - `short_name` is the short name searched for
    ///  - `long_name` is the long name searched for
    /// At least one of the parameters is required to be [`Some`]
    fn remove(
        &mut self,
        short_name: Option<&str>,
        long_name: Option<&str>,
    ) -> (Option<FlagArgument<T, E>>, Option<FlagArgument<T, E>>) {
        if short_name.is_none() && long_name.is_none() {
            return (None, None);
        }

        let mut short = None;
        let mut long = None;

        let mut i = 0;
        while i < self.0.len() {
            match (short_name, self.0[i].short_name()) {
                (Some(short_name), Some(f_short_name)) => {
                    if short_name == f_short_name {
                        assert!(short.is_none());
                        short = Some(self.0.swap_remove(i));
                        continue;
                    }
                }
                _ => {}
            }

            match (long_name, self.0[i].long_name()) {
                (Some(long_name), Some(f_long_name)) => {
                    if long_name == f_long_name {
                        assert!(long.is_none());
                        long = Some(self.0.swap_remove(i));
                        continue;
                    }
                }
                _ => {}
            }

            i += 1;
        }

        if long.is_some() && short.is_none() {
            (long, short)
        } else {
            (short, long)
        }
    }
}

impl<T, E> Deref for FlagSet<T, E> {
    type Target = [FlagArgument<T, E>];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'a, T, E> IntoIterator for &'a FlagSet<T, E> {
    type Item = &'a FlagArgument<T, E>;
    type IntoIter = std::slice::Iter<'a, FlagArgument<T, E>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, E> IntoIterator for &'a mut FlagSet<T, E> {
    type Item = &'a mut FlagArgument<T, E>;
    type IntoIter = std::slice::IterMut<'a, FlagArgument<T, E>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
