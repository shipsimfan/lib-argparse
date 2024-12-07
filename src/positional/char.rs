use crate::{Argument, Error, InvalidCharError, Positional, PositionalInfo, PositionalResult};

impl Positional for char {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        match argument.as_str()?.parse() {
            Ok(value) => *this = Some(value),
            Err(error) => {
                return PositionalResult::Error(Error::invalid_positional_value(
                    info.value,
                    InvalidCharError::from(error),
                ))
            }
        }
        PositionalResult::Next
    }
}
