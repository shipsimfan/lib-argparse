use crate::FlagArgument;
use std::{marker::PhantomData, str::FromStr};

/// A flag argument which takes one parameter and parses it to a type using [`FromStr`] and then
/// calls an action with it.
pub struct ParsingFlagArgument<
    Options: 'static,
    T: FromStr<Err = ParseError>,
    ParseError: std::fmt::Display,
    Action: Fn(&mut Options, T) -> Result<(), ActionError>,
    ActionError: std::fmt::Display,
> {
    /// The name to follow the short prefix
    short_name: Option<&'static str>,

    /// The name to follow the long prefix
    long_name: Option<&'static str>,

    /// The message displayed if the parameter is missing
    missing_parameter: &'static str,

    /// Is this flag repeatable?
    repeatable: bool,

    /// The message to display if this flag is required
    required: Option<&'static str>,

    /// The action to be called upon matching
    action: Action,

    /// The hint to be displayed in the help
    hint: Option<&'static str>,

    /// The description to be displayed in the help
    description: Option<&'static [&'static str]>,

    phantom_options: PhantomData<Options>,
    phantom_t: PhantomData<T>,
    phantom_error: PhantomData<ParseError>,
}

impl<
        Options,
        T: FromStr<Err = ParseError>,
        ParseError: std::fmt::Display,
        Action: Fn(&mut Options, T) -> Result<(), ActionError>,
        ActionError: std::fmt::Display,
    > ParsingFlagArgument<Options, T, ParseError, Action, ActionError>
{
    /// Creates a new [`ParsingFlagArgument`]
    pub const fn new(missing_parameter: &'static str, action: Action) -> Self {
        ParsingFlagArgument {
            short_name: None,
            long_name: None,
            missing_parameter,
            repeatable: false,
            required: None,
            action,
            hint: None,
            description: None,
            phantom_options: PhantomData,
            phantom_t: PhantomData,
            phantom_error: PhantomData,
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
    pub const fn hint(mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets if the description to be displayed in the help
    ///
    /// Each value in the slice will be dislayed on its own line
    pub const fn description(mut self, description: &'static [&'static str]) -> Self {
        self.description = Some(description);
        self
    }
}

impl<
        Options,
        T: FromStr<Err = ParseError>,
        ParseError: std::fmt::Display,
        Action: Fn(&mut Options, T) -> Result<(), ActionError>,
        ActionError: std::fmt::Display,
    > FlagArgument<Options> for ParsingFlagArgument<Options, T, ParseError, Action, ActionError>
{
    fn short_name(&self) -> Option<&str> {
        self.short_name
    }

    fn long_name(&self) -> Option<&str> {
        self.long_name
    }

    fn count(&self) -> usize {
        1
    }

    fn action(
        &self,
        options: &mut Options,
        mut parameters: Vec<std::ffi::OsString>,
    ) -> crate::Result<()> {
        if parameters.len() != 1 {
            return Err(crate::Error::missing_parameters(self.missing_parameter));
        }

        (self.action)(
            options,
            T::from_str(
                &parameters
                    .swap_remove(0)
                    .into_string()
                    .map_err(|_| crate::Error::invalid_utf8())?,
            )
            .map_err(|error| crate::Error::custom(error.to_string()))?,
        )
        .map_err(|error| crate::Error::custom(error.to_string()))
    }

    fn finalize(&self, ran: bool) -> crate::Result<()> {
        if let Some(message) = self.required {
            if !ran {
                return Err(crate::Error::missing_required(message));
            }
        }

        Ok(())
    }

    fn repeatable(&self) -> bool {
        self.repeatable
    }

    fn hint(&self) -> Option<&str> {
        self.hint
    }

    fn description(&self) -> Option<&[&str]> {
        self.description
    }
}
