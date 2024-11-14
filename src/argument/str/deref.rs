use crate::StrArgument;
use std::ops::Deref;

impl<'a> Deref for StrArgument<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            StrArgument::Borrowed(borrowed) => borrowed,
            StrArgument::Owned(owned) => owned,
        }
    }
}

impl<'a> AsRef<str> for StrArgument<'a> {
    fn as_ref(&self) -> &str {
        match self {
            StrArgument::Borrowed(borrowed) => borrowed,
            StrArgument::Owned(owned) => owned,
        }
    }
}
