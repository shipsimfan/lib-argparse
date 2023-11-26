use crate::FlagArgument;
use std::{ffi::OsString, marker::PhantomData};

/// A simple flag argument that can take zero or more parameters and pass them into an action
pub struct SimpleFlagArgument<
    Options: 'static,
    Error: std::fmt::Display + 'static,
    Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
> {
    /// The name to follow the short prefix
    short_name: Option<&'static str>,

    /// The name to follow the long prefix
    long_name: Option<&'static str>,

    /// The number of parameters to accept
    count: usize,

    /// The message displayed if less than `count` parameters are passed
    missing_parameters: &'static str,

    /// Is this flag repeatable?
    repeatable: bool,

    /// The message to display if this flag is required
    required: Option<&'static str>,

    /// The action to be called upon matching
    action: Action,

    /// The hint to be displayed in the help
    hint: Option<&'static dyn std::fmt::Display>,

    /// The description to be displayed in the help
    description: Option<&'static [&'static dyn std::fmt::Display]>,

    phantom: PhantomData<Options>,
}

impl<
        Options,
        Error: std::fmt::Display,
        Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
    > SimpleFlagArgument<Options, Error, Action>
{
    /// Creates a new [`SimpleFlagArgument`]
    ///
    /// This structure guarantees that exactly `count` parameters will be passed to `action`
    pub const fn new(count: usize, missing_parameters: &'static str, action: Action) -> Self {
        SimpleFlagArgument {
            short_name: None,
            long_name: None,
            count,
            missing_parameters,
            repeatable: false,
            required: None,
            action,
            hint: None,
            description: None,
            phantom: PhantomData,
        }
    }

    /// Sets the name which follows the short prefix
    pub const fn short_name(mut self, short_name: &'static str) -> Self {
        self.short_name = Some(short_name);
        self
    }

    /// Sets the name which follows the long prefix
    pub const fn long_name(mut self, long_name: &'static str) -> Self {
        self.long_name = Some(long_name);
        self
    }

    /// Sets if this flag is repeatable
    pub const fn repeatable(mut self, repeatable: bool) -> Self {
        self.repeatable = repeatable;
        self
    }

    /// Sets if this flag is required and the message to be displayed if it is
    pub const fn required(mut self, required: Option<&'static str>) -> Self {
        self.required = required;
        self
    }

    /// Sets if the hint to be displayed in the help
    pub const fn hint(mut self, hint: &'static dyn std::fmt::Display) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets if the description to be displayed in the help
    ///
    /// Each value in the slice will be dislayed on its own line
    pub const fn description(
        mut self,
        description: &'static [&'static dyn std::fmt::Display],
    ) -> Self {
        self.description = Some(description);
        self
    }
}

impl<
        Options,
        Error: std::fmt::Display,
        Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
    > FlagArgument<Options> for SimpleFlagArgument<Options, Error, Action>
{
    fn short_name(&self) -> Option<&str> {
        self.short_name
    }

    fn long_name(&self) -> Option<&str> {
        self.long_name
    }

    fn count(&self) -> usize {
        self.count
    }

    fn repeatable(&self) -> bool {
        self.repeatable
    }

    fn action(&self, options: &mut Options, parameters: Vec<OsString>) -> crate::Result<()> {
        if parameters.len() != self.count {
            return Err(crate::Error::missing_parameters(self.missing_parameters));
        }

        (self.action)(options, parameters).map_err(|error| crate::Error::custom(error.to_string()))
    }

    fn finalize(&self, ran: bool) -> crate::Result<()> {
        if let Some(message) = self.required {
            if !ran {
                return Err(crate::Error::missing_required(message));
            }
        }

        Ok(())
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        self.hint
    }

    fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
        self.description
    }
}
