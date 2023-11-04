use crate::{Error, PositionalParser};
use std::{borrow::Cow, ffi::OsString, marker::PhantomData, str::FromStr};

/// Parses values using the [`std::str::FromStr`] trait
pub struct SimplePositionalParserTry<T: FromStr + 'static, O, F>
where
    F: Fn(&mut O, T),
{
    /// Function called when parsing is complete
    callback: F,

    /// Determines if this positional is required
    ///
    /// The contained string holds the error message if no value is provided
    required: Option<Cow<'static, str>>,

    /// The usage hint display in help
    hint: Cow<'static, str>,

    /// Variable for storing if a value was parsed
    recieved_value: bool,

    phantom_type: PhantomData<T>,
    phantom_options: PhantomData<O>,
}

/// Parses values using the [`From<String>`] trait
pub struct SimplePositionalParser<T: From<String> + 'static, O, F, E>
where
    F: Fn(&mut O, T),
{
    /// Function called when parsing is complete
    callback: F,

    /// Determines if this positional is required
    ///
    /// The contained string holds the error message if no value is provided
    required: Option<Cow<'static, str>>,

    /// The usage hint display in help
    hint: Cow<'static, str>,

    /// Variable for storing if a value was parsed
    recieved_value: bool,

    phantom_type: PhantomData<T>,
    phantom_options: PhantomData<O>,
    phantom_error: PhantomData<E>,
}

/// Parses values using the [`From<OsString>`] trait
pub struct SimplePositionalParserOS<T: From<OsString> + 'static, O, F, E>
where
    F: Fn(&mut O, T),
{
    /// Function called when parsing is complete
    callback: F,

    /// Determines if this positional is required
    ///
    /// The contained string holds the error message if no value is provided
    required: Option<Cow<'static, str>>,

    /// The usage hint display in help
    hint: Cow<'static, str>,

    /// Variable for storing if a value was parsed
    recieved_value: bool,

    phantom_type: PhantomData<T>,
    phantom_options: PhantomData<O>,
    phantom_error: PhantomData<E>,
}

impl<T: FromStr, O, F> SimplePositionalParserTry<T, O, F>
where
    F: Fn(&mut O, T),
{
    /// Creates a new [`SimplePositionalParserTry`]
    ///
    ///  - `callback` is the function called to store the parsed value
    ///  - `hint` is the usage hint displayed in help
    pub fn new<S: Into<Cow<'static, str>>>(callback: F, hint: S) -> Self {
        SimplePositionalParserTry {
            callback,
            required: None,
            hint: hint.into(),

            recieved_value: false,

            phantom_type: PhantomData,
            phantom_options: PhantomData,
        }
    }

    /// Sets this flag to be required
    ///
    ///  - `message` is the error message used if no value is provided
    pub fn set_required<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
        self.required = Some(message.into());
        self
    }

    /// Sets this flag to be not required
    pub fn set_not_required(mut self) -> Self {
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

impl<T: FromStr, O, F> PositionalParser for SimplePositionalParserTry<T, O, F>
where
    F: Fn(&mut O, T),
{
    type Options = O;
    type Error = T::Err;

    fn parse(
        &mut self,
        options: &mut Self::Options,
        argument: std::ffi::OsString,
    ) -> Result<bool, Error<T::Err>> {
        self.recieved_value = true;
        let value = T::from_str(
            &argument
                .into_string()
                .map_err(|string| Error::InvalidUTF8(string))?,
        )
        .map_err(|error| Error::Other(error))?;
        (self.callback)(options, value);
        Ok(false)
    }

    fn finalize(&mut self, _: &mut Self::Options) -> Result<(), Error<T::Err>> {
        match (&self.required, self.recieved_value) {
            (Some(message), false) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        self.recieved_value = false;
        Ok(())
    }

    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        if self.required.is_none() {
            write!(f, "[")?;
        }

        write!(f, "{}", self.hint)?;

        if self.required.is_none() {
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl<T: From<String>, O, F, E> SimplePositionalParser<T, O, F, E>
where
    F: Fn(&mut O, T),
{
    /// Creates a new [`SimplePositionalParser`]
    ///
    ///  - `callback` is the function called to store the parsed value
    ///  - `hint` is the usage hint displayed in help
    pub fn new<S: Into<Cow<'static, str>>>(callback: F, hint: S) -> Self {
        SimplePositionalParser {
            callback,
            required: None,
            hint: hint.into(),

            recieved_value: false,

            phantom_type: PhantomData,
            phantom_options: PhantomData,
            phantom_error: PhantomData,
        }
    }

    /// Sets this flag to be required
    ///
    ///  - `message` is the error message used if no value is provided
    pub fn set_required<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
        self.required = Some(message.into());
        self
    }

    /// Sets this flag to be not required
    pub fn set_not_required(mut self) -> Self {
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

impl<T: From<String>, O, F, E> PositionalParser for SimplePositionalParser<T, O, F, E>
where
    F: Fn(&mut O, T),
{
    type Options = O;
    type Error = E;

    fn parse(
        &mut self,
        options: &mut Self::Options,
        argument: std::ffi::OsString,
    ) -> Result<bool, Error<E>> {
        self.recieved_value = true;
        let value = T::from(
            argument
                .into_string()
                .map_err(|string| Error::InvalidUTF8(string))?,
        );
        (self.callback)(options, value);
        Ok(false)
    }

    fn finalize(&mut self, _: &mut Self::Options) -> Result<(), Error<E>> {
        match (&self.required, self.recieved_value) {
            (Some(message), false) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        self.recieved_value = false;
        Ok(())
    }

    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        if self.required.is_none() {
            write!(f, "[")?;
        }

        write!(f, "{}", self.hint)?;

        if self.required.is_none() {
            write!(f, "]")?;
        }

        Ok(())
    }
}

impl<T: From<OsString>, O, F, E> SimplePositionalParserOS<T, O, F, E>
where
    F: Fn(&mut O, T),
{
    /// Creates a new [`SimplePositionalParserOS`]
    ///
    ///  - `callback` is the function called to store the parsed value
    ///  - `hint` is the usage hint displayed in help
    pub fn new<S: Into<Cow<'static, str>>>(callback: F, hint: S) -> Self {
        SimplePositionalParserOS {
            callback,
            required: None,
            hint: hint.into(),

            recieved_value: false,

            phantom_type: PhantomData,
            phantom_options: PhantomData,
            phantom_error: PhantomData,
        }
    }

    /// Sets this flag to be required
    ///
    ///  - `message` is the error message used if no value is provided
    pub fn set_required<S: Into<Cow<'static, str>>>(mut self, message: S) -> Self {
        self.required = Some(message.into());
        self
    }

    /// Sets this flag to be not required
    pub fn set_not_required(mut self) -> Self {
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

impl<T: From<OsString>, O, F, E> PositionalParser for SimplePositionalParserOS<T, O, F, E>
where
    F: Fn(&mut O, T),
{
    type Options = O;
    type Error = E;

    fn parse(
        &mut self,
        options: &mut Self::Options,
        argument: std::ffi::OsString,
    ) -> Result<bool, Error<E>> {
        self.recieved_value = true;
        let value = T::from(argument);
        (self.callback)(options, value);
        Ok(false)
    }

    fn finalize(&mut self, _: &mut Self::Options) -> Result<(), Error<E>> {
        match (&self.required, self.recieved_value) {
            (Some(message), false) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        self.recieved_value = false;
        Ok(())
    }

    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " ")?;

        if self.required.is_none() {
            write!(f, "[")?;
        }

        write!(f, "{}", self.hint)?;

        if self.required.is_none() {
            write!(f, "]")?;
        }

        Ok(())
    }
}
