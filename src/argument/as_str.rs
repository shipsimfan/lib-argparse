use crate::{Argument, Result};

impl<'a> Argument<'a> {
    /// Attempts to get this argument a string
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Argument::OsStr(os) => os.as_str(),
            Argument::Str(str) => Ok(str),
        }
    }
}
