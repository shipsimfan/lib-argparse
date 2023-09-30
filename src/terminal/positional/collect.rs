use crate::{Error, PositionalParser};
use std::{borrow::Cow, ffi::OsString, marker::PhantomData};

/// Collects the remaining positional arguments as a `Vec<OsString>`
pub struct CollectOsPositionalParser<E: 'static, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    /// If this positional requires at least one value
    ///
    /// The contained string is the error message if no value is present
    required: Option<Cow<'static, str>>,

    /// The callback for updating the options
    callback: F,

    /// The list of `OsString`s collected during parsing
    list: Vec<OsString>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

/// Collects the remaining positional arguments as a `Vec<String>`
pub struct CollectPositionalParser<E: 'static, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    /// If this positional requires at least one value
    ///
    /// The contained string is the error message if no value is present
    required: Option<Cow<'static, str>>,

    /// The callback for updating the options
    callback: F,

    /// The list of `String`s collected during parsing
    list: Vec<String>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

impl<E, T, F> CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    /// Creates a new `CollectOsPositionalParser`
    ///
    /// `callback` is called upon completion of parsing to update the options
    pub fn new(callback: F) -> Self {
        CollectOsPositionalParser {
            required: None,
            callback,

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    /// Sets this positional argument to require at least one value
    ///
    /// If no value is provided, `error_message` is the error message used
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, error_message: S) {
        self.required = Some(error_message.into());
    }

    /// Allows this positional argument to take no value
    pub fn clear_required(&mut self) {
        self.required = None;
    }
}

impl<E, T, F> PositionalParser for CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    type Options = T;
    type Error = E;

    fn parse(&mut self, _: &mut Self::Options, arg: OsString) -> Result<bool, Error<E>> {
        self.list.push(arg);
        Ok(true)
    }

    fn finalize(&mut self, options: &mut Self::Options) -> Result<(), Error<E>> {
        let mut strings = Vec::new();
        std::mem::swap(&mut self.list, &mut strings);

        match (&self.required, strings.len()) {
            (Some(message), 0) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        (self.callback)(options, strings).map_err(|error| Error::Other(error))
    }
}

impl<E, T, F> CollectPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    /// Creates a new `CollectPositionalParser`
    ///
    /// `callback` is called upon completion of parsing to update the options
    pub fn new(callback: F) -> Self {
        CollectPositionalParser {
            required: None,
            callback,

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    /// Sets this positional argument to require at least one value
    ///
    /// If no value is provided, `error_message` is the error message used
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, error_message: S) {
        self.required = Some(error_message.into());
    }

    /// Allows this positional argument to take no value
    pub fn clear_required(&mut self) {
        self.required = None;
    }
}

impl<E, T, F> PositionalParser for CollectPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    type Options = T;
    type Error = E;

    fn parse(&mut self, _: &mut Self::Options, arg: OsString) -> Result<bool, Error<E>> {
        self.list.push(
            arg.into_string()
                .map_err(|string| Error::InvalidUTF8(string))?,
        );
        Ok(true)
    }

    fn finalize(&mut self, options: &mut Self::Options) -> Result<(), Error<E>> {
        let mut strings = Vec::new();
        std::mem::swap(&mut self.list, &mut strings);

        match (&self.required, strings.len()) {
            (Some(message), 0) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        (self.callback)(options, strings).map_err(|error| Error::Other(error))
    }
}
