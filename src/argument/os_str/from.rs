use crate::OsStrArgument;
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

impl<'a> From<&'a OsStr> for OsStrArgument<'a> {
    fn from(value: &'a OsStr) -> Self {
        OsStrArgument::Borrowed(value)
    }
}

impl<'a> From<OsString> for OsStrArgument<'a> {
    fn from(value: OsString) -> Self {
        OsStrArgument::Owned(value)
    }
}

impl<'a> From<Cow<'a, OsStr>> for OsStrArgument<'a> {
    fn from(value: Cow<'a, OsStr>) -> Self {
        match value {
            Cow::Borrowed(borrowed) => OsStrArgument::Borrowed(borrowed),
            Cow::Owned(owned) => OsStrArgument::Owned(owned),
        }
    }
}
