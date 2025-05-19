use crate::{Argument, Error, InvalidCharError, Positional, PositionalInfo, PositionalResult};

impl Positional for char {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
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
