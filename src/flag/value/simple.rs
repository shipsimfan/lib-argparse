use crate::{Error, ValueParser};
use std::{borrow::Cow, marker::PhantomData, str::FromStr};

/// Parses values using the `std::str::FromStr` trait
pub struct SimpleValueParser<T: FromStr + 'static> {
    missing_message: Cow<'static, str>,
    phantom: PhantomData<T>,
}

impl<T: FromStr> SimpleValueParser<T> {
    /// Creates a new SimpleValueParser
    ///
    /// `missing_message` is the error message if the parameter is missing
    pub fn new<S: Into<Cow<'static, str>>>(missing_message: S) -> Self {
        SimpleValueParser {
            missing_message: missing_message.into(),
            phantom: PhantomData,
        }
    }
}

impl<T: FromStr> ValueParser for SimpleValueParser<T> {
    type Value = T;
    type Error = Error<T::Err>;

    fn parse(&mut self, args: &mut crate::ArgStream) -> Result<Self::Value, Self::Error> {
        T::from_str(
            &args
                .next()?
                .ok_or_else(|| Error::MissingParameter(self.missing_message.clone()))?,
        )
        .map_err(|error| Error::Other(error))
    }
}
