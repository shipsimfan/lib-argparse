use crate::StrArgument;
use std::borrow::Cow;

impl<'a> From<&'a str> for StrArgument<'a> {
    fn from(value: &'a str) -> Self {
        StrArgument::Borrowed(value)
    }
}

impl<'a> From<String> for StrArgument<'a> {
    fn from(value: String) -> Self {
        StrArgument::Owned(value)
    }
}

impl<'a> From<Cow<'a, str>> for StrArgument<'a> {
    fn from(value: Cow<'a, str>) -> Self {
        match value {
            Cow::Borrowed(borrowed) => StrArgument::Borrowed(borrowed),
            Cow::Owned(owned) => StrArgument::Owned(owned),
        }
    }
}
