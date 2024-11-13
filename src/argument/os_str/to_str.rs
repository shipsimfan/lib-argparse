use crate::{Error, OsStrArgument};
use std::borrow::Cow;

impl<'a> OsStrArgument<'a> {
    /// Converts this argument to a string, returning an error if it contains invalid UTF-8
    pub fn to_str(&self) -> Result<&str, Error> {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => owned.as_os_str(),
        }
        .to_str()
        .ok_or(Error::InvalidUTF8(match self.to_string_lossy() {
            Cow::Borrowed(borrowed) => borrowed.to_owned(),
            Cow::Owned(owned) => owned,
        }))
    }
}
