use crate::{Error, ValueParser};
use std::{borrow::Cow, marker::PhantomData, str::FromStr};

/// Parses values using the [`std::str::FromStr`] trait
pub struct SimpleValueParserTry<T: FromStr + 'static> {
    /// Error message if the value is missing
    missing_message: Cow<'static, str>,

    phantom: PhantomData<T>,
}

impl<T: FromStr> SimpleValueParserTry<T> {
    /// Creates a new [`SimpleValueParser`]
    ///
    ///  - `missing_message` is the error message if the parameter is missing
    pub fn new<S: Into<Cow<'static, str>>>(missing_message: S) -> Self {
        SimpleValueParserTry {
            missing_message: missing_message.into(),
            phantom: PhantomData,
        }
    }
}

impl<T: FromStr> ValueParser for SimpleValueParserTry<T> {
    type Value = T;
    type Error = T::Err;

    fn parse(&mut self, args: &mut crate::ArgStream) -> Result<Self::Value, Error<Self::Error>> {
        T::from_str(
            &args
                .next()?
                .ok_or_else(|| Error::MissingParameter(self.missing_message.clone()))?,
        )
        .map_err(|error| Error::Other(error))
    }
}
