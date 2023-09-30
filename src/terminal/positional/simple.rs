use crate::{Error, PositionalParser};
use std::{borrow::Cow, marker::PhantomData, str::FromStr};

/// Parses values using the `std::str::FromStr` trait
pub struct SimplePositionalParser<T: FromStr + 'static, O, F>
where
    F: Fn(&mut O, T),
{
    /// Function called when parsing is complete
    callback: F,
    /// Determines if this positional is required
    ///
    /// The contained string holds the error message if no value is provided
    required: Option<Cow<'static, str>>,

    /// Variable for storing if a value was parsed
    recieved_value: bool,

    phantom_type: PhantomData<T>,
    phantom_options: PhantomData<O>,
}

impl<T: FromStr, O, F> SimplePositionalParser<T, O, F>
where
    F: Fn(&mut O, T),
{
    /// Creates a new SimpleValueParser
    ///
    /// `callback` is the function called to store the parsed value
    pub fn new(callback: F) -> Self {
        SimplePositionalParser {
            callback,
            required: None,

            recieved_value: false,

            phantom_type: PhantomData,
            phantom_options: PhantomData,
        }
    }

    /// Sets this flag to be required
    ///
    /// `message` is the error message used if no value is provided
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, message: S) {
        self.required = Some(message.into());
    }

    /// Sets this flag to be not required
    pub fn set_not_required(&mut self) {
        self.required = None;
    }
}

impl<T: FromStr, O, F> PositionalParser for SimplePositionalParser<T, O, F>
where
    F: Fn(&mut O, T),
{
    type Options = O;
    type Error = T::Err;

    fn parse(
        &mut self,
        options: &mut Self::Options,
        arg: std::ffi::OsString,
    ) -> Result<bool, Error<T::Err>> {
        self.recieved_value = true;
        let value = T::from_str(
            &arg.into_string()
                .map_err(|string| Error::InvalidUTF8(string))?,
        )
        .map_err(|error| Error::Other(error))?;
        (self.callback)(options, value);
        Ok(false)
    }

    fn finish(&mut self, _: &mut Self::Options) -> Result<(), Error<T::Err>> {
        match (&self.required, self.recieved_value) {
            (Some(message), false) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        self.recieved_value = false;
        Ok(())
    }
}
