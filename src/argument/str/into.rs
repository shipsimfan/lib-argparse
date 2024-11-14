use crate::StrArgument;
use std::borrow::Cow;

impl<'a> Into<String> for StrArgument<'a> {
    fn into(self) -> String {
        self.into_owned()
    }
}

impl<'a> Into<Cow<'a, str>> for StrArgument<'a> {
    fn into(self) -> Cow<'a, str> {
        match self {
            StrArgument::Borrowed(borrowed) => Cow::Borrowed(borrowed),
            StrArgument::Owned(owned) => Cow::Owned(owned),
        }
    }
}
