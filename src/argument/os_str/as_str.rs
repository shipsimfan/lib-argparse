use crate::{Error, OsStrArgument};

impl<'a> OsStrArgument<'a> {
    /// Converts this argument to a string, returning an error if it contains invalid UTF-8
    pub fn as_str(&self) -> Result<&str, Error> {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => owned.as_os_str(),
        }
        .to_str()
        .ok_or(Error::invalid_utf8(self.to_string_lossy()))
    }
}
