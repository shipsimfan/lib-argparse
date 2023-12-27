use crate::FlagArgument;
use std::{marker::PhantomData, str::FromStr};

/// A flag argument which takes one parameter and parses it to a type using [`FromStr`] and then
/// calls an action with it.
pub struct ParsingFlagArgument<
    'a,
    Options: 'a,
    T: FromStr<Err = ParseError>,
    ParseError: std::fmt::Display,
    Action: Fn(&mut Options, T) -> Result<(), ActionError>,
    ActionError: std::fmt::Display,
> {
    /// The name to follow the short prefix
    short_name: Option<&'a str>,

    /// The name to follow the long prefix
    long_name: Option<&'a str>,

    /// The message displayed if the parameter is missing
    missing_parameter: &'a dyn std::fmt::Display,

    /// Is this flag repeatable?
    repeatable: bool,

    /// The message to display if this flag is required
    required: Option<&'a dyn std::fmt::Display>,

    /// The action to be called upon matching
    action: Action,

    /// The flag group this flag belongs to
    group: Option<&'a str>,

    /// The hint to be displayed in the help
    hint: Option<&'a dyn std::fmt::Display>,

    /// The description to be displayed in the help
    description: Option<&'a [&'a dyn std::fmt::Display]>,

    phantom_options: PhantomData<Options>,
    phantom_t: PhantomData<T>,
    phantom_error: PhantomData<ParseError>,
}

impl<
        'a,
        Options,
        T: FromStr<Err = ParseError>,
        ParseError: std::fmt::Display,
        Action: Fn(&mut Options, T) -> Result<(), ActionError>,
        ActionError: std::fmt::Display,
    > ParsingFlagArgument<'a, Options, T, ParseError, Action, ActionError>
{
    /// Creates a new [`ParsingFlagArgument`]
    pub const fn new(missing_parameter: &'a dyn std::fmt::Display, action: Action) -> Self {
        ParsingFlagArgument {
            short_name: None,
            long_name: None,
            missing_parameter,
            repeatable: false,
            required: None,
            action,
            group: None,
            hint: None,
            description: None,
            phantom_options: PhantomData,
            phantom_t: PhantomData,
            phantom_error: PhantomData,
        }
    }

    /// Sets the name which follows the short prefix
    pub const fn short_name(mut self, short_name: &'a str) -> Self {
        self.short_name = Some(short_name);
        self
    }

    /// Sets the name which follows the long prefix
    pub const fn long_name(mut self, long_name: &'a str) -> Self {
        self.long_name = Some(long_name);
        self
    }

    /// Sets if this flag is repeatable
    pub const fn repeatable(mut self, repeatable: bool) -> Self {
        self.repeatable = repeatable;
        self
    }

    /// Sets if this flag is required and the message to be displayed if it is
    pub const fn required(mut self, required: Option<&'a dyn std::fmt::Display>) -> Self {
        self.required = required;
        self
    }

    /// Sets the flag group this flag is apart of
    pub const fn group(mut self, group: &'a str) -> Self {
        self.group = Some(group);
        self
    }

    /// Sets if the hint to be displayed in the help
    pub const fn hint<S: std::fmt::Display>(mut self, hint: &'a S) -> Self {
        self.hint = Some(hint);
        self
    }

    /// Sets if the description to be displayed in the help
    ///
    /// Each value in the slice will be dislayed on its own line
    pub const fn description(mut self, description: &'a [&'a dyn std::fmt::Display]) -> Self {
        self.description = Some(description);
        self
    }
}

impl<
        'a,
        Options,
        T: FromStr<Err = ParseError>,
        ParseError: std::fmt::Display,
        Action: Fn(&mut Options, T) -> Result<(), ActionError>,
        ActionError: std::fmt::Display,
    > FlagArgument<'a, Options>
    for ParsingFlagArgument<'a, Options, T, ParseError, Action, ActionError>
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

    fn action(&self, options: &mut Options, mut parameters: Vec<String>) -> crate::Result<()> {
        if parameters.len() != 1 {
            return Err(crate::Error::missing_parameters(
                self.missing_parameter.to_string(),
            ));
        }

        (self.action)(
            options,
            T::from_str(&parameters.swap_remove(0))
                .map_err(|error| crate::Error::custom(error.to_string()))?,
        )
        .map_err(|error| crate::Error::custom(error.to_string()))
    }

    fn finalize(&self, ran: bool) -> crate::Result<()> {
        if let Some(message) = self.required {
            if !ran {
                return Err(crate::Error::missing_required(message.to_string()));
            }
        }

        Ok(())
    }

    fn repeatable(&self) -> bool {
        self.repeatable
    }

    fn group(&self) -> Option<&str> {
        self.group
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        self.hint
    }

    fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
        self.description
    }
}

#[cfg(test)]
mod tests {
    use crate::{FlagArgument, FlagClass, ParsingFlagArgument};

    #[test]
    fn parsing_flag() {
        const SHORT_NAME: &str = "t";
        const LONG_NAME: &str = "test";
        const GROUP: &str = "EXAMPLE";
        const HINT: &str = "VALUE";
        const DESCRIPTION: &str = "Example";

        let mut parsing_flag =
            ParsingFlagArgument::<(), usize, _, _, &str>::new(&"", |_, value| {
                if value == 123 {
                    Ok(())
                } else {
                    Err("invalid number")
                }
            });

        assert_eq!(FlagArgument::<()>::short_name(&parsing_flag), None);
        assert_eq!(FlagArgument::<()>::long_name(&parsing_flag), None);
        assert_eq!(FlagArgument::<()>::group(&parsing_flag), None);
        assert_eq!(FlagArgument::<()>::repeatable(&parsing_flag), false);
        assert!(FlagArgument::<()>::hint(&parsing_flag).is_none());
        assert!(FlagArgument::<()>::description(&parsing_flag).is_none());

        assert_eq!(FlagArgument::<()>::class(&parsing_flag), FlagClass::Normal);
        assert_eq!(FlagArgument::<()>::count(&parsing_flag), 1);

        assert!(FlagArgument::<()>::action(&parsing_flag, &mut (), Vec::new()).is_err());
        assert!(
            FlagArgument::<()>::action(&parsing_flag, &mut (), vec!["test".to_owned()]).is_err()
        );
        assert!(
            FlagArgument::<()>::action(&parsing_flag, &mut (), vec!["256".to_owned()]).is_err()
        );
        assert!(FlagArgument::<()>::action(&parsing_flag, &mut (), vec!["123".to_owned()]).is_ok());

        assert!(FlagArgument::<()>::finalize(&parsing_flag, false).is_ok());
        assert!(FlagArgument::<()>::finalize(&parsing_flag, true).is_ok());

        parsing_flag = parsing_flag
            .short_name(&SHORT_NAME)
            .long_name(&LONG_NAME)
            .repeatable(true)
            .required(Some(&"required"))
            .group(GROUP)
            .hint(&HINT)
            .description(&[&DESCRIPTION]);

        assert_eq!(
            FlagArgument::<()>::short_name(&parsing_flag),
            Some(SHORT_NAME)
        );
        assert_eq!(
            FlagArgument::<()>::long_name(&parsing_flag),
            Some(LONG_NAME)
        );
        assert_eq!(FlagArgument::<()>::group(&parsing_flag), Some(GROUP));
        assert_eq!(FlagArgument::<()>::repeatable(&parsing_flag), true);
        assert!(FlagArgument::<()>::hint(&parsing_flag).is_some());
        assert!(FlagArgument::<()>::description(&parsing_flag).is_some());

        assert!(FlagArgument::<()>::finalize(&parsing_flag, false).is_err());
        assert!(FlagArgument::<()>::finalize(&parsing_flag, true).is_ok());
    }
}
