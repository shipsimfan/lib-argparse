use crate::{ArgStream, Error, FlagArgument, FlagKind};
use std::borrow::Cow;

/// Flag which calls an function upon matching
pub struct ActionFlag<T, E> {
    /// The function to be called upon matching
    action: Box<dyn Fn(&mut T, Vec<String>) -> Result<(), E>>,

    /// Message for if a paramter is missing
    missing_error_message: Cow<'static, str>,

    /// The number of required arguments for the action
    count: usize,
}

impl<T, E> ActionFlag<T, E> {
    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T)`
    ///  - `description` is the description of this argument displayed in the help
    pub fn simple<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T) + 'static,
        description: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, _| Ok(action(options))),
                missing_error_message: "".into(),
                count: 0,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T) -> Result<(), E>`
    ///  - `description` is the description of this argument displayed in the help
    pub fn simple_res<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T) -> Result<(), E> + 'static,
        description: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, _| action(options)),
                missing_error_message: "".into(),
                count: 0,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T, &str)`
    ///  - `missing_parameter_message` is the message for if the parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn str<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, &str) + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, strings| Ok(action(options, &strings[0]))),
                missing_error_message: missing_parameter_message.into(),
                count: 1,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T, &str) -> Result<(), E>`
    ///  - `missing_parameter_message` is the message for if the parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn str_res<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, &str) -> Result<(), E> + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, strings| action(options, &strings[0])),
                missing_error_message: missing_parameter_message.into(),
                count: 1,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T, String)`
    ///  - `missing_parameter_message` is the message for if the parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn string<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, String) + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, mut strings| {
                    Ok(action(options, strings.swap_remove(0)))
                }),
                missing_error_message: missing_parameter_message.into(),
                count: 1,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `action` is in the form `fn(&mut T, String) -> Result<(), E>`
    ///  - `missing_parameter_message` is the message for if the parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn string_res<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, String) -> Result<(), E> + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, mut strings| {
                    action(options, strings.swap_remove(0))
                }),
                missing_error_message: missing_parameter_message.into(),
                count: 1,
            }),
            description,
        )
    }

    /// Creates an [`ActionFlag`]
    ///
    ///  - `count` is the number of arguments to pull from the stream
    ///  - `action` is in the form `fn(&mut T, Vec<String>)`
    ///  - `missing_parameter_message` is the message for if a parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn vec_string<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, strings| Ok(action(options, strings))),
                missing_error_message: missing_parameter_message.into(),
                count,
            }),
            description,
        )
    }

    /// Creates and [`ActionFlag`]
    ///
    ///  - `count` is the number of parameters to pull from the stream
    ///  - `action` is in the form `fn(&mut T, Vec<String>) -> Result<(), E>`
    ///  - `missing_parameter_message` is the message for if a parameter is missing during the parse
    ///  - `description` is the description of this argument displayed in the help
    pub fn vec_string_res<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>>(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) -> Result<(), E> + 'static,
        missing_parameter_message: S1,
        description: S2,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Action(ActionFlag {
                action: Box::new(move |options, strings| action(options, strings)),
                missing_error_message: missing_parameter_message.into(),
                count,
            }),
            description,
        )
    }

    /// Parses this flag from the stream
    ///
    ///  - `options` is the developer defined options
    ///  - `args` is the argument stream
    pub(super) fn parse(&mut self, options: &mut T, args: &mut ArgStream) -> Result<(), Error<E>> {
        let parameters = self.collect_parameters(args)?;

        (self.action)(options, parameters).map_err(|error| Error::Other(error))
    }

    /// Collects `self.count` parameters from the argument stream
    ///
    ///  - `args` is the argument stream
    fn collect_parameters(&mut self, args: &mut ArgStream) -> Result<Vec<String>, Error<E>> {
        let mut parameters = Vec::with_capacity(self.count);

        for _ in 0..self.count {
            parameters.push(
                args.next()?
                    .ok_or_else(|| Error::MissingParameter(self.missing_error_message.clone()))?,
            );
        }

        Ok(parameters)
    }
}
