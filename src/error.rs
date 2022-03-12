#[derive(Debug)]
pub enum ArgumentParseError {
    UnknownArgument(String),
    UnexpectedArgument(String),
    MultipleDefinition(String),
    InvalidNumber(String, usize, usize),
    TooFewArguments(String, usize, usize),
    MissingRequiredArgument(String),
    UserArgumentError(Box<dyn std::error::Error>),
}

impl std::error::Error for ArgumentParseError {}

impl std::fmt::Display for ArgumentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentParseError::UnknownArgument(arg) => write!(f, "Unknown argument \"{}\"", arg),
            ArgumentParseError::UnexpectedArgument(arg) => {
                write!(f, "Unexpected argument \"{}\"", arg)
            }
            ArgumentParseError::MultipleDefinition(arg) => {
                write!(f, "Unable to set multiple values for \"{}\"", arg)
            }
            ArgumentParseError::InvalidNumber(arg, expected, real) => {
                if *real == 0 {
                    write!(f, "No argument for {}", arg)
                } else {
                    write!(
                        f,
                        "Expected {} argument{} for {}, instead found {}",
                        expected,
                        if *expected == 1 { "" } else { "s" },
                        arg,
                        real
                    )
                }
            }
            ArgumentParseError::TooFewArguments(arg, expected, real) => {
                if *real == 0 {
                    write!(f, "No argument for {}", arg)
                } else {
                    write!(
                        f,
                        "Only {} argument{} for {}, expected at least {}",
                        real,
                        if *expected == 1 { "" } else { "s" },
                        arg,
                        expected,
                    )
                }
            }
            ArgumentParseError::MissingRequiredArgument(arg) => {
                write!(f, "Missing argument for {}", arg)
            }
            ArgumentParseError::UserArgumentError(error) => write!(f, "{}", error),
        }
    }
}

impl From<Box<dyn std::error::Error>> for ArgumentParseError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        ArgumentParseError::UserArgumentError(error)
    }
}
