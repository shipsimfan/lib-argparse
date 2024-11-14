use crate::{ArgumentSource, Result};

/// A command which can be parsed from an [`ArgumentSource`]
pub trait Command: Sized {
    /// Parse this command from `source`
    fn parse(source: &mut dyn ArgumentSource) -> Result<Self>;

    /// Parse this command from the environment arguments
    fn parse_env() -> Result<Self> {
        Self::parse(&mut std::env::args_os())
    }
}
