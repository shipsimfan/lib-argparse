use crate::Argument;
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

impl<'a> From<&'a str> for Argument<'a> {
    fn from(value: &'a str) -> Self {
        Argument::Str(value.into())
    }
}

impl<'a> From<String> for Argument<'a> {
    fn from(value: String) -> Self {
        Argument::Str(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for Argument<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        Argument::Str(value.into())
    }
}

impl<'a> From<&'a OsStr> for Argument<'a> {
    fn from(value: &'a OsStr) -> Self {
        Argument::OsStr(value.into())
    }
}

impl<'a> From<OsString> for Argument<'a> {
    fn from(value: OsString) -> Self {
        Argument::OsStr(value.into())
    }
}

impl<'a> From<Cow<'a, OsStr>> for Argument<'a> {
    fn from(value: Cow<'a, OsStr>) -> Self {
        Argument::OsStr(value.into())
    }
}
