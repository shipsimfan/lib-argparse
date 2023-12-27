use crate::FlagArgument;
use std::{ffi::OsString, marker::PhantomData};

/// A simple flag argument that can take zero or more parameters and pass them into an action
pub struct SimpleFlagArgument<
    'a,
    Options: 'a,
    Error: std::fmt::Display + 'a,
    Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
> {
    /// The name to follow the short prefix
    short_name: Option<&'a str>,

    /// The name to follow the long prefix
    long_name: Option<&'a str>,

    /// The number of parameters to accept
    count: usize,

    /// The message displayed if less than `count` parameters are passed
    missing_parameters: &'a dyn std::fmt::Display,

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

    phantom: PhantomData<Options>,
}

impl<
        'a,
        Options,
        Error: std::fmt::Display,
        Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
    > SimpleFlagArgument<'a, Options, Error, Action>
{
    /// Creates a new [`SimpleFlagArgument`]
    ///
    /// This structure guarantees that exactly `count` parameters will be passed to `action`
    pub const fn new(
        count: usize,
        missing_parameters: &'a dyn std::fmt::Display,
        action: Action,
    ) -> Self {
        SimpleFlagArgument {
            short_name: None,
            long_name: None,
            count,
            missing_parameters,
            repeatable: false,
            required: None,
            action,
            group: None,
            hint: None,
            description: None,
            phantom: PhantomData,
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
        Error: std::fmt::Display,
        Action: Fn(&mut Options, Vec<OsString>) -> Result<(), Error>,
    > FlagArgument<'a, Options> for SimpleFlagArgument<'a, Options, Error, Action>
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

    fn action(&self, options: &mut Options, parameters: Vec<String>) -> crate::Result<()> {
        self.action_os(
            options,
            parameters
                .into_iter()
                .map(|parameter| OsString::from(parameter))
                .collect(),
        )
    }

    fn action_os(&self, options: &mut Options, parameters: Vec<OsString>) -> crate::Result<()> {
        if parameters.len() != self.count {
            return Err(crate::Error::missing_parameters(
                self.missing_parameters.to_string(),
            ));
        }

        (self.action)(options, parameters).map_err(|error| crate::Error::custom(error.to_string()))
    }

    fn finalize(&self, ran: bool) -> crate::Result<()> {
        if let Some(message) = self.required {
            if !ran {
                return Err(crate::Error::missing_required(message.to_string()));
            }
        }

        Ok(())
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
    use crate::{FlagArgument, FlagClass, SimpleFlagArgument};

    #[test]
    fn simple_flag() {
        const COUNT: usize = 2;
        const SHORT_NAME: &str = "t";
        const LONG_NAME: &str = "test";
        const GROUP: &str = "EXAMPLE";
        const HINT: &str = "VAL1 VAL2";
        const DESCRIPTION: &str = "Example";

        let mut simple_flag = SimpleFlagArgument::<(), _, _>::new(COUNT, &"", |_, parameters| {
            if parameters.len() != COUNT {
                Err("invalid parameters")
            } else {
                Ok(())
            }
        });

        assert_eq!(FlagArgument::<()>::short_name(&simple_flag), None);
        assert_eq!(FlagArgument::<()>::long_name(&simple_flag), None);
        assert_eq!(FlagArgument::<()>::group(&simple_flag), None);
        assert_eq!(FlagArgument::<()>::repeatable(&simple_flag), false);
        assert!(FlagArgument::<()>::hint(&simple_flag).is_none());
        assert!(FlagArgument::<()>::description(&simple_flag).is_none());

        assert_eq!(FlagArgument::<()>::class(&simple_flag), FlagClass::Normal);
        assert_eq!(FlagArgument::<()>::count(&simple_flag), COUNT);

        let mut parameters = Vec::new();
        for _ in 0..COUNT {
            assert!(FlagArgument::<()>::action(&simple_flag, &mut (), parameters.clone()).is_err());
            parameters.push(String::new());
        }
        assert!(FlagArgument::<()>::action(&simple_flag, &mut (), parameters.clone()).is_ok());

        assert!(FlagArgument::<()>::finalize(&simple_flag, false).is_ok());
        assert!(FlagArgument::<()>::finalize(&simple_flag, true).is_ok());

        simple_flag = simple_flag
            .short_name(&SHORT_NAME)
            .long_name(&LONG_NAME)
            .repeatable(true)
            .required(Some(&""))
            .group(&GROUP)
            .hint(&HINT)
            .description(&[&DESCRIPTION]);

        assert_eq!(
            FlagArgument::<()>::short_name(&simple_flag),
            Some(SHORT_NAME)
        );
        assert_eq!(FlagArgument::<()>::long_name(&simple_flag), Some(LONG_NAME));
        assert_eq!(FlagArgument::<()>::group(&simple_flag), Some(GROUP));
        assert_eq!(FlagArgument::<()>::repeatable(&simple_flag), true);
        assert!(FlagArgument::<()>::hint(&simple_flag).is_some());
        assert!(FlagArgument::<()>::description(&simple_flag).is_some());

        assert!(FlagArgument::<()>::finalize(&simple_flag, false).is_err());
        assert!(FlagArgument::<()>::finalize(&simple_flag, true).is_ok());
    }
}
