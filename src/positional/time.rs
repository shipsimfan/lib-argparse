use crate::{Argument, Error, InvalidDurationError, Positional, PositionalInfo, PositionalResult};
use std::time::Duration;

impl Positional for Duration {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut duration = None;
        f64::parse(&mut duration, argument, &info.drop_default())?;

        let duration = duration.unwrap();
        if duration.is_sign_negative() || duration.is_infinite() || duration.is_nan() {
            return PositionalResult::Error(Error::invalid_positional_value(
                info.value,
                InvalidDurationError::new(duration),
            ));
        }

        *this = Some(Duration::from_secs_f64(duration));
        PositionalResult::Next
    }
}
