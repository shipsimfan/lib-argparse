use crate::OsStrArgument;
use std::ffi::OsString;

impl<'a> OsStrArgument<'a> {
    /// Converts this argument into an owned version
    pub fn into_owned(self) -> OsString {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed.to_os_string(),
            OsStrArgument::Owned(owned) => owned,
        }
    }
}
