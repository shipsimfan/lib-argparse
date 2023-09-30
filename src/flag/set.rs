use crate::{Error, FlagArgument};
use std::ops::Deref;

/// Represents a set of flag arguments
///
/// No two flags in this structure can share either a short or long name
pub(crate) struct FlagSet<T, E: 'static>(Vec<FlagArgument<T, E>>);

impl<T, E> FlagSet<T, E> {
    /// Creates a new empty `FlagSet`
    pub(crate) fn new() -> Self {
        FlagSet(Vec::new())
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// The iterator yields all items from start to end.
    pub(crate) fn iter_mut(&mut self) -> std::slice::IterMut<FlagArgument<T, E>> {
        self.0.iter_mut()
    }

    /// Gets an argument based on its short name
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

    /// Pushes `flag_argument` into the set
    ///
    /// Returns any arguments which have conflicting names
    ///
    /// If there is only one conflict, the conflicting argument will always be the first value of the returned tuple
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

    pub(crate) fn finalize(&mut self) -> Result<(), Error<E>> {
        for flag_argument in self {
            flag_argument.finalize()?;
        }

        Ok(())
    }

    /// Removes any arguments that have the same long or short name as `long_name` or `short_name` respectively
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

impl<'a, T, E> IntoIterator for &'a mut FlagSet<T, E> {
    type Item = &'a mut FlagArgument<T, E>;
    type IntoIter = std::slice::IterMut<'a, FlagArgument<T, E>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
