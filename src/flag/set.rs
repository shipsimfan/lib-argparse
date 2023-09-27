use crate::FlagArgument;
use std::ops::Deref;

pub(crate) struct FlagSet<T, E>(Vec<FlagArgument<T, E>>);

impl<T, E> FlagSet<T, E> {
    pub(crate) fn new() -> Self {
        FlagSet(Vec::new())
    }

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
