use std::{
    env::{args_os, ArgsOs},
    ffi::{OsStr, OsString},
    iter::Peekable,
};

pub struct InvalidUTF8(OsString);

pub struct ArgStream {
    args: Peekable<ArgsOs>,
}

impl ArgStream {
    pub(crate) fn new() -> Self {
        let args = args_os().peekable();

        ArgStream { args }
    }

    pub fn peek(&mut self) -> Result<Option<&str>, InvalidUTF8> {
        match self.peek_os() {
            Some(arg) => arg
                .to_str()
                .map(|str| Some(str))
                .ok_or_else(|| InvalidUTF8(arg.to_owned())),
            None => Ok(None),
        }
    }

    pub fn peek_os(&mut self) -> Option<&OsStr> {
        self.args.peek().map(|string| string.as_os_str())
    }

    pub fn next(&mut self) -> Result<Option<String>, InvalidUTF8> {
        match self.next_os() {
            Some(arg) => arg
                .into_string()
                .map(|string| Some(string))
                .map_err(|os_string| InvalidUTF8(os_string)),
            None => Ok(None),
        }
    }

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
