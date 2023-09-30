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
    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T)`
    pub fn simple(action: impl Fn(&mut T) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| Ok(action(options))),
            missing_error_message: "".into(),
            count: 0,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T) -> Result<(), E>`
    pub fn simple_res(action: impl Fn(&mut T) -> Result<(), E> + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| action(options)),
            missing_error_message: "".into(),
            count: 0,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, &str)`
    /// `missing_parameter_message` is the message for if the parameter is missing during the parse
    pub fn str<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, &str) + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, &strings[0]))),
            missing_error_message: missing_parameter_message.into(),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, &str) -> Result<(), E>`
    /// `missing_parameter_message` is the message for if the parameter is missing during the parse
    pub fn str_res<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, &str) -> Result<(), E> + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, &strings[0])),
            missing_error_message: missing_parameter_message.into(),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, String)`
    pub fn string<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, String) + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| {
                Ok(action(options, strings.swap_remove(0)))
            }),
            missing_error_message: missing_parameter_message.into(),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, String) -> Result<(), E>`
    pub fn string_res<S: Into<Cow<'static, str>>>(
        action: impl Fn(&mut T, String) -> Result<(), E> + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| action(options, strings.swap_remove(0))),
            missing_error_message: missing_parameter_message.into(),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, Vec<String>)`
    pub fn vec_string<S: Into<Cow<'static, str>>>(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, strings))),
            missing_error_message: missing_parameter_message.into(),
            count,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, Vec<String>) -> Result<(), E>`
    pub fn vec_string_res<S: Into<Cow<'static, str>>>(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) -> Result<(), E> + 'static,
        missing_parameter_message: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, strings)),
            missing_error_message: missing_parameter_message.into(),
            count,
        }))
    }

    pub(super) fn parse(&mut self, options: &mut T, args: &mut ArgStream) -> Result<(), Error<E>> {
        let parameters = self.collect_parameters(args)?;

        (self.action)(options, parameters).map_err(|error| Error::Other(error))
    }

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
