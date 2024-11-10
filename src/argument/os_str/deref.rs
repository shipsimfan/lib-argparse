use crate::OsStrArgument;
use std::{ffi::OsStr, ops::Deref};

impl<'a> Deref for OsStrArgument<'a> {
    type Target = OsStr;

    fn deref(&self) -> &Self::Target {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => &owned,
        }
    }
}

impl<'a> AsRef<OsStr> for OsStrArgument<'a> {
    fn as_ref(&self) -> &OsStr {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => &owned,
        }
    }
}
