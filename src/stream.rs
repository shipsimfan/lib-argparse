use std::{
    borrow::Cow,
    env::args_os,
    ffi::{OsStr, OsString},
    iter::Peekable,
};

use crate::Error;

/// A stream of arguments
pub enum ArgStream {
    OsString(Peekable<Box<dyn Iterator<Item = OsString>>>),
    String(Peekable<Box<dyn Iterator<Item = String>>>),
}

impl ArgStream {
    /// Creates a new [`ArgStream`] from the [`String`] Iterator `I`
    pub(crate) fn new<I: IntoIterator<Item = String> + 'static>(iter: I) -> Self {
        let iter: Box<dyn Iterator<Item = String>> = Box::new(iter.into_iter());

        ArgStream::String(iter.peekable())
    }

    /// Creates a new [`ArgStream`] from the [`OsString`] Iterator `I`
    pub(crate) fn new_os<I: IntoIterator<Item = OsString> + 'static>(iter: I) -> Self {
        let iter: Box<dyn Iterator<Item = OsString>> = Box::new(iter.into_iter());

        ArgStream::OsString(iter.peekable())
    }

    /// Creates a new [`ArgStream`] from OS arguments
    pub(crate) fn new_env() -> Self {
        ArgStream::new_os(args_os())
    }

    /// Returns the next [`str`] in the stream without advancing the stream
    pub fn peek<E>(&mut self) -> Result<Option<&str>, Error<E>> {
        match self {
            ArgStream::String(stream) => Ok(stream.peek().map(|string| string.as_str())),
            ArgStream::OsString(stream) => match stream.peek() {
                Some(arg) => arg
                    .to_str()
                    .map(|str| Some(str))
                    .ok_or_else(|| Error::InvalidUTF8(arg.to_owned())),
                None => Ok(None),
            },
        }
    }

    /// Returns the next [`OsStr`] in the stream without advancing the stream
    pub fn peek_os(&mut self) -> Option<Cow<OsStr>> {
        match self {
            ArgStream::OsString(stream) => stream.peek().map(|string| string.as_os_str().into()),
            ArgStream::String(stream) => stream.peek().map(|string| OsString::from(string).into()),
        }
    }

    /// Returns the next [`String`] in the stream and advances the stream
    pub fn next<E>(&mut self) -> Result<Option<String>, Error<E>> {
        match self {
            ArgStream::String(stream) => Ok(stream.next()),
            ArgStream::OsString(stream) => match stream.next() {
                Some(arg) => arg
                    .into_string()
                    .map(|string| Some(string))
                    .map_err(|os_string| Error::InvalidUTF8(os_string)),
                None => Ok(None),
            },
        }
    }

    /// Returns the next [`OsString`] in the stream and advances the stream
    pub fn next_os(&mut self) -> Option<OsString> {
        match self {
            ArgStream::OsString(stream) => stream.next(),
            ArgStream::String(stream) => stream.next().map(|string| OsString::from(string)),
        }
    }
}
