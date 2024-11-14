use crate::StrArgument;

impl<'a> StrArgument<'a> {
    /// Gets an owned version of this argument
    pub fn to_owned(&self) -> String {
        match self {
            StrArgument::Borrowed(borrowed) => borrowed,
            StrArgument::Owned(owned) => owned.as_str(),
        }
        .to_string()
    }

    /// Converts this argument into an owned version
    pub fn into_owned(self) -> String {
        match self {
            StrArgument::Borrowed(borrowed) => borrowed.to_string(),
            StrArgument::Owned(owned) => owned,
        }
    }
}
