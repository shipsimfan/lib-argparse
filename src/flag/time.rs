use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidDurationError, Result};
use std::time::Duration;

impl Flag for Duration {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let mut duration = None;
        f64::parse(&mut duration, source, &info.drop_default(), long)?;

        let duration = duration.unwrap();
        if duration.is_sign_negative() || duration.is_infinite() || duration.is_nan() {
            return Err(Error::invalid_flag_value(
                info,
                long,
                InvalidDurationError::new(duration),
            ));
        }

        *this = Some(Duration::from_secs_f64(duration));
        Ok(())
    }
}
