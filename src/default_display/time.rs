use crate::DefaultDisplay;
use std::time::Duration;

pub struct DurationDisplay(f64);

impl DefaultDisplay for Duration {
    type Display<'a> = DurationDisplay;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        DurationDisplay(self.as_secs_f64())
    }
}

impl std::fmt::Display for DurationDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}s", self.0)
    }
}
