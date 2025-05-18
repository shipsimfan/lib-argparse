use crate::Error;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidUTF8(string) => write!(f, "invalid UTF-8 \"{string}\""),
            Error::MissingArgument(argument) => write!(f, "missing argument \"{argument}\""),
            Error::MissingPositionalValue(value) => write!(f, "missing \"{value}\""),
            Error::InvalidPositionalValue(value, error) => {
                write!(f, "invalid \"{value}\" - {error}")
            }
            Error::MissingFlagValue(argument, value) => {
                write!(f, "missing \"{value}\" for \"{argument}\"")
            }
            Error::InvalidFlagValue(argument, value, error) => {
                write!(f, "invalid \"{value}\" for \"{argument}\" - {error}")
            }
            Error::RepeatedFlag(argument) => write!(f, "repeated flag \"{argument}\""),
            Error::UnknownArgument(argument) => write!(f, "unknown argument \"{argument}\""),
            Error::Custom(message) => message.fmt(f),
        }
    }
}
