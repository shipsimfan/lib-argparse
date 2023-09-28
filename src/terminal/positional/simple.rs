use crate::{Error, PositionalParser};
use std::{borrow::Cow, marker::PhantomData, str::FromStr};

/// Parses values using the `std::str::FromStr` trait
pub struct SimplePositionalParser<T: FromStr + 'static, O, F>
where
    F: Fn(&mut O, T),
{
    callback: F,
    required: Option<Cow<'static, str>>,

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

    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, message: S) {
        self.required = Some(message.into());
    }

    pub fn set_not_required(&mut self) {
        self.required = None;
    }
}

impl<T: FromStr, O, F> PositionalParser for SimplePositionalParser<T, O, F>
where
    F: Fn(&mut O, T),
{
    type Options = O;
    type Error = Error<T::Err>;

    fn parse(
        &mut self,
        options: &mut Self::Options,
        arg: std::ffi::OsString,
    ) -> Result<bool, Self::Error> {
        self.recieved_value = true;
        let value = T::from_str(&arg.into_string().map_err(|_| Error::InvalidUTF8)?)
            .map_err(|error| Error::Other(error))?;
        (self.callback)(options, value);
        Ok(false)
    }

    fn finish(&mut self, _: &mut Self::Options) -> Result<(), Self::Error> {
        match (&self.required, self.recieved_value) {
            (Some(message), false) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        self.recieved_value = false;
        Ok(())
    }
}
