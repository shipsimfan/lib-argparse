use crate::OsStrArgument;

impl<'a> OsStrArgument<'a> {
    pub fn to_str(&self) -> Result<&str, Error> {
        match self {
            OsStrArgument::Borrowed(borrowed) => borrowed,
            OsStrArgument::Owned(owned) => owned.as_os_str(),
        }
        .to_str()
        .ok_or(Error::InvalidUTF8(self.to_owned()))
    }
}
