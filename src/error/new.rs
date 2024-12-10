use crate::{flag::DEFUALT_FLAG_VALUE, Error, Flag, FlagInfo};

impl Error {
    /// Create an [`Error::Custom`] containing `message`
    pub fn custom<T: 'static + std::error::Error>(message: T) -> Self {
        Error::Custom(Box::new(message))
    }

    /// Create an [`Error::InvalidUTF8`] for `string`
    pub fn invalid_utf8<T: Into<String>>(string: T) -> Self {
        Error::InvalidUTF8(string.into())
    }

    /// Create an [`Error::MissingArgument`] for `argument`
    pub fn missing_argument(argument: &'static str) -> Self {
        Error::MissingArgument(argument)
    }

    /// Create an [`Error::MissingPositionalValue`] for `value`
    pub fn missing_positional_value(value: &'static str) -> Self {
        Error::MissingPositionalValue(value)
    }

    /// Create an [`Error::InvalidPositionalValue`] for `value`
    pub fn invalid_positional_value<T: 'static + std::error::Error>(
        value: &'static str,
        error: T,
    ) -> Self {
        Error::InvalidPositionalValue(value, Box::new(error))
    }

    /// Create an [`Error::MissingFlagValue`] for `value` of `argument`
    pub fn missing_flag_value<T: Flag>(info: &FlagInfo<T>, long: bool) -> Self {
        Error::MissingFlagValue(
            if long {
                info.long_name
            } else {
                info.short_name
            }
            .unwrap(),
            info.value.unwrap_or(DEFUALT_FLAG_VALUE),
        )
    }

    /// Create an [`Error::InvalidFlagValue`] for `value` of `argument`
    pub fn invalid_flag_value<T: Flag, E: 'static + std::error::Error>(
        info: &FlagInfo<T>,
        long: bool,
        error: E,
    ) -> Self {
        Error::InvalidFlagValue(
            if long {
                info.long_name
            } else {
                info.short_name
            }
            .unwrap(),
            info.value.unwrap_or(DEFUALT_FLAG_VALUE),
            Box::new(error),
        )
    }

    /// Create an [`Error::UnknownArgument`] for `argument`
    pub fn unknown_argument(argument: String) -> Self {
        Error::UnknownArgument(argument)
    }
}
