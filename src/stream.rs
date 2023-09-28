use std::{
    env::{args_os, ArgsOs},
    ffi::{OsStr, OsString},
    iter::Peekable,
};

pub struct InvalidUTF8(OsString);

/// A stream of arguments
pub struct ArgStream {
    args: Peekable<ArgsOs>,
}

impl ArgStream {
    /// Creates a new stream from OS arguments
    pub(crate) fn new() -> Self {
        let args = args_os().peekable();

        ArgStream { args }
    }

    /// Returns the next `str` in the stream without advancing the stream
    pub fn peek(&mut self) -> Result<Option<&str>, InvalidUTF8> {
        match self.peek_os() {
            Some(arg) => arg
                .to_str()
                .map(|str| Some(str))
                .ok_or_else(|| InvalidUTF8(arg.to_owned())),
            None => Ok(None),
        }
    }

    /// Returns the next `OsStr` in the stream without advancing the stream
    pub fn peek_os(&mut self) -> Option<&OsStr> {
        self.args.peek().map(|string| string.as_os_str())
    }

    /// Returns the next `String` in the stream and advances the stream
    pub fn next(&mut self) -> Result<Option<String>, InvalidUTF8> {
        match self.next_os() {
            Some(arg) => arg
                .into_string()
                .map(|string| Some(string))
                .map_err(|os_string| InvalidUTF8(os_string)),
            None => Ok(None),
        }
    }

    /// Returns the next `OsString` in the stream and advances the stream
    pub fn next_os(&mut self) -> Option<OsString> {
        self.args.next()
    }
}

impl InvalidUTF8 {
    pub fn os_str(&self) -> &OsStr {
        &self.0
    }

    pub fn unwrap(self) -> OsString {
        self.0
    }
}

impl std::error::Error for InvalidUTF8 {}

impl std::fmt::Display for InvalidUTF8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid UTF-8 {:?}", self.0)
    }
}

impl std::fmt::Debug for InvalidUTF8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
