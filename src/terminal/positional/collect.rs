use crate::{Error, PositionalParser};
use std::{borrow::Cow, ffi::OsString, marker::PhantomData};

pub struct CollectOsPositionalParser<E: 'static, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    required: Option<Cow<'static, str>>,
    callback: F,

    list: Vec<OsString>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

pub struct CollectPositionalParser<E: 'static, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    required: Option<Cow<'static, str>>,
    callback: F,

    list: Vec<String>,

    phantom_error: PhantomData<E>,
    phantom_type: PhantomData<T>,
}

impl<E, T, F> CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    pub fn new(callback: F) -> Self {
        CollectOsPositionalParser {
            required: None,
            callback,

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, error_message: S) {
        self.required = Some(error_message.into());
    }

    pub fn clear_required(&mut self) {
        self.required = None;
    }
}

impl<E, T, F> PositionalParser for CollectOsPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<OsString>) -> Result<(), E>,
{
    type Options = T;
    type Error = Error<E>;

    fn parse(&mut self, _: &mut Self::Options, arg: OsString) -> Result<bool, Self::Error> {
        self.list.push(arg);
        Ok(true)
    }

    fn finish(&mut self, options: &mut Self::Options) -> Result<(), Self::Error> {
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
    pub fn new(callback: F) -> Self {
        CollectPositionalParser {
            required: None,
            callback,

            list: Vec::new(),
            phantom_error: PhantomData,
            phantom_type: PhantomData,
        }
    }

    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, error_message: S) {
        self.required = Some(error_message.into());
    }

    pub fn clear_required(&mut self) {
        self.required = None;
    }
}

impl<E, T, F> PositionalParser for CollectPositionalParser<E, T, F>
where
    F: Fn(&mut T, Vec<String>) -> Result<(), E>,
{
    type Options = T;
    type Error = Error<E>;

    fn parse(&mut self, _: &mut Self::Options, arg: OsString) -> Result<bool, Self::Error> {
        self.list
            .push(arg.into_string().map_err(|_| Error::InvalidUTF8)?);
        Ok(true)
    }

    fn finish(&mut self, options: &mut Self::Options) -> Result<(), Self::Error> {
        let mut strings = Vec::new();
        std::mem::swap(&mut self.list, &mut strings);

        match (&self.required, strings.len()) {
            (Some(message), 0) => return Err(Error::MissingParameter(message.clone())),
            _ => {}
        }

        (self.callback)(options, strings).map_err(|error| Error::Other(error))
    }
}
