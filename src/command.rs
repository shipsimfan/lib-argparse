use crate::{ArgsOsSource, ArgumentSource, Result};

/// A command which can be parsed from an [`ArgumentSource`]
pub trait Command: Sized {
    /// Parse this command from `source`
    ///
    /// Returns [`None`] if a flag or argument triggers an output without result, like help or version
    fn parse(source: &mut dyn ArgumentSource) -> Result<Option<Self>>;

    /// Parse this command from the environment arguments
    ///
    /// Returns [`None`] if a flag or argument triggers an output without result, like help or version
    fn parse_env() -> Result<Option<Self>> {
        Self::parse(&mut ArgsOsSource::new())
    }
}
