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

    /// The usage hint for help
    hint: Cow<'static, str>,

    /// The list of [`OsString`]s collected during parsing
    list: Vec<OsString>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

/// Collects the remaining positional arguments as a [`Vec<String>`]
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

    /// The usage hint for help
    hint: Cow<'static, str>,

    /// The list of [`String`]s collected during parsing
    list: Vec<String>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

impl<E, T, F> CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    /// Creates a new [`CollectOsPositionalParser`]
    ///
    ///  - `callback` is called upon completion of parsing to update the options
    ///  - `hint` is the usage hint displayed in help
    pub fn new<S: Into<Cow<'static, str>>>(callback: F, hint: S) -> Self {
        CollectOsPositionalParser {
            required: None,
            callback,
            hint: hint.into(),

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    /// Sets this positional argument to require at least one value
    ///
    /// - `error_message` is the error message used if there are no values
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, error_message: S) {
        self.required = Some(error_message.into());
    }

    /// Allows this positional argument to take no value
    pub fn clear_required(&mut self) {
        self.required = None;
    }

    /// Sets the usage hint displayed during help
    ///
    ///  - `hint` is the string the hint will be set to
    pub fn set_hint<S: Into<Cow<'static, str>>>(&mut self, hint: S) {
        self.hint = hint.into();
    }
}

impl<E, T, F> PositionalParser for CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    type Options = T;
    type Error = E;

    fn parse(&mut self, _: &mut Self::Options, argument: OsString) -> Result<bool, Error<E>> {
        self.list.push(argument);
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

    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        if self.required.is_none() {
            write!(f, "[")?;
        }

        write!(f, "{}..", self.hint)?;

        if self.required.is_none() {
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl<E, T, F> CollectPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    /// Creates a new [`CollectPositionalParser`]
    ///
    ///  - `callback` is called upon completion of parsing to update the options
    ///  - `hint` is the usage hint displayed in help
    pub fn new<S: Into<Cow<'static, str>>>(callback: F, hint: S) -> Self {
        CollectPositionalParser {
            required: None,
            callback,
            hint: hint.into(),

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    /// Sets this positional argument to require at least one value
    ///
    /// - `error_message` is the error message used if there are no values
    pub fn set_required<S: Into<Cow<'static, str>>>(mut self, error_message: S) -> Self {
        self.required = Some(error_message.into());
        self
    }

    /// Allows this positional argument to take no value
    pub fn clear_required(mut self) -> Self {
        self.required = None;
        self
    }

    /// Sets the usage hint displayed during help
    ///
    ///  - `hint` is the string the hint will be set to
    pub fn set_hint<S: Into<Cow<'static, str>>>(mut self, hint: S) -> Self {
        self.hint = hint.into();
        self
    }
}

impl<E, T, F> PositionalParser for CollectPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    type Options = T;
    type Error = E;

    fn parse(&mut self, _: &mut Self::Options, argument: OsString) -> Result<bool, Error<E>> {
        self.list.push(
            argument
                .into_string()
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

    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        if self.required.is_none() {
            write!(f, "[")?;
        }

        write!(f, "{}..", self.hint)?;

        if self.required.is_none() {
            write!(f, "]")?;
        }

        Ok(())
    }
}
