use crate::{Error, ValueParser};
use std::{borrow::Cow, ffi::OsString, marker::PhantomData};

/// Parses values using the [`From<OsString>`] trait
pub struct SimpleValueParserOS<T: From<OsString> + 'static, E: 'static> {
    /// Error message if the value is missing
    missing_message: Cow<'static, str>,

    phantom_type: PhantomData<T>,
    phantom_error: PhantomData<E>,
}

impl<T: From<OsString>, E> SimpleValueParserOS<T, E> {
    /// Creates a new [`SimpleValueParser`]
    ///
    ///  - `missing_message` is the error message if the parameter is missing
    pub fn new<S: Into<Cow<'static, str>>>(missing_message: S) -> Self {
        SimpleValueParserOS {
            missing_message: missing_message.into(),

            phantom_type: PhantomData,
            phantom_error: PhantomData,
        }
    }
}

impl<T: From<OsString>, E> ValueParser for SimpleValueParserOS<T, E> {
    type Value = T;
    type Error = E;

    fn parse(&mut self, args: &mut crate::ArgStream) -> Result<Self::Value, Error<Self::Error>> {
        Ok(T::from(args.next_os().ok_or_else(|| {
            Error::MissingParameter(self.missing_message.clone())
        })?))
    }
}
