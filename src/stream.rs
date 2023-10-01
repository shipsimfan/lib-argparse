use std::{
    env::{args_os, ArgsOs},
    ffi::{OsStr, OsString},
    iter::Peekable,
};

use crate::Error;

/// A stream of arguments
pub struct ArgStream {
    args: Peekable<ArgsOs>,
}

impl ArgStream {
    /// Creates a new [`ArgStream`] from OS arguments
    pub(crate) fn new() -> Self {
        let args = args_os().peekable();

        ArgStream { args }
    }

    /// Returns the next [`str`] in the stream without advancing the stream
    pub fn peek<E>(&mut self) -> Result<Option<&str>, Error<E>> {
        match self.peek_os() {
            Some(arg) => arg
                .to_str()
                .map(|str| Some(str))
                .ok_or_else(|| Error::InvalidUTF8(arg.to_owned())),
            None => Ok(None),
        }
    }

    /// Returns the next [`OsStr`] in the stream without advancing the stream
    pub fn peek_os(&mut self) -> Option<&OsStr> {
        self.args.peek().map(|string| string.as_os_str())
    }

    /// Returns the next [`String`] in the stream and advances the stream
    pub fn next<E>(&mut self) -> Result<Option<String>, Error<E>> {
        match self.next_os() {
            Some(arg) => arg
                .into_string()
                .map(|string| Some(string))
                .map_err(|os_string| Error::InvalidUTF8(os_string)),
            None => Ok(None),
        }
    }

    /// Returns the next [`OsString`] in the stream and advances the stream
    pub fn next_os(&mut self) -> Option<OsString> {
        self.args.next()
    }
}
