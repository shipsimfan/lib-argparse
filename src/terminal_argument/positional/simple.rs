use crate::{Error, PositionalArgument, Result};
use std::{ffi::OsString, marker::PhantomData, num::NonZeroUsize};

/// A positional argument that takes upto a fixed number of arguments and passes them to an action
pub struct SimplePositionalArgument<
    'a,
    Options: 'a,
    Action: Fn(&mut Options, usize, OsString) -> Result<'a, ()>,
> {
    name: &'a dyn std::fmt::Display,
    description: &'a [&'a dyn std::fmt::Display],
    hint: Option<&'a dyn std::fmt::Display>,

    count: NonZeroUsize,

    minimum: Option<(NonZeroUsize, &'a dyn std::fmt::Display)>,

    action: Action,

    phantom: PhantomData<Options>,
}

impl<'a, Options: 'a, Action: Fn(&mut Options, usize, OsString) -> Result<'a, ()>>
    SimplePositionalArgument<'a, Options, Action>
{
    /// Creates a new [`SimplePositionalArgument`]
    pub const fn new(name: &'a dyn std::fmt::Display, count: NonZeroUsize, action: Action) -> Self {
        SimplePositionalArgument {
            name,
            description: &[],
            hint: None,
            count,
            minimum: None,
            action,
            phantom: PhantomData,
        }
    }

    /// Sets the name for this positional argument
    pub const fn name(mut self, name: &'a dyn std::fmt::Display) -> Self {
        self.name = name;
        self
    }

    /// Sets the description for this positional argument
    pub const fn description(mut self, description: &'a [&'a dyn std::fmt::Display]) -> Self {
        self.description = description;
        self
    }

    /// Sets the hint that will be displayed in the help
    pub const fn hint(mut self, hint: &'a dyn std::fmt::Display) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets the maximum number of arguments to accept for this positional
    pub const fn count(mut self, count: NonZeroUsize) -> Self {
        self.count = count;
        self
    }

    /// Sets this positional to not be required
    pub const fn set_not_required(mut self) -> Self {
        self.minimum = None;
        self
    }

    /// Sets this positional to require at least one value
    pub const fn set_required(self, missing: &'a dyn std::fmt::Display) -> Self {
        self.set_minimum(unsafe { NonZeroUsize::new_unchecked(1) }, missing)
    }

    /// Sets this positional to require `minimum` values
    pub const fn set_minimum(
        mut self,
        minimum: NonZeroUsize,
        missing: &'a dyn std::fmt::Display,
    ) -> Self {
        self.minimum = Some((minimum, missing));
        self
    }
}

impl<'a, Options: 'a, Action: Fn(&mut Options, usize, OsString) -> Result<'a, ()>>
    PositionalArgument<'a, Options> for SimplePositionalArgument<'a, Options, Action>
{
    fn name(&self) -> &dyn std::fmt::Display {
        self.name
    }

    fn description(&self) -> &[&dyn std::fmt::Display] {
        self.description
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        self.hint
    }

    fn count(&self) -> std::num::NonZeroUsize {
        self.count
    }

    fn action(&self, options: &mut Options, index: usize, parameter: OsString) -> Result<'a, ()> {
        (self.action)(options, index, parameter)
    }

    fn finalize(&self, count: usize) -> Result<'a, ()> {
        if let Some((minimum, missing)) = self.minimum {
            if count < minimum.get() {
                return Err(Error::missing_required(missing.to_string()));
            }
        }

        Ok(())
    }
}
