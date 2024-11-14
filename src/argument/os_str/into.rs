use crate::OsStrArgument;
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

impl<'a> Into<OsString> for OsStrArgument<'a> {
    fn into(self) -> OsString {
        self.into_owned()
    }
}

impl<'a> Into<Cow<'a, OsStr>> for OsStrArgument<'a> {
    fn into(self) -> Cow<'a, OsStr> {
        match self {
            OsStrArgument::Borrowed(borrowed) => Cow::Borrowed(borrowed),
            OsStrArgument::Owned(owned) => Cow::Owned(owned),
        }
    }
}
