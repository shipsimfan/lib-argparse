use crate::OsStrArgument;
use std::ffi::OsString;

impl<'a> OsStrArgument<'a> {
    /// Gets an owned version of this argument
    pub fn to_owned(&self) -> OsString {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => owned.as_os_str(),
        }
        .to_os_string()
    }

    /// Converts this argument into an owned version
    pub fn into_owned(self) -> OsString {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed.to_os_string(),
            OsStrArgument::Owned(owned) => owned,
        }
    }
}
