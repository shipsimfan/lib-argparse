use crate::{ArgumentSource, Result};

/// A type which can parsed from a flag
pub trait Flag: Sized {
    /// Parse arguments from `source` to get the value
    fn parse(source: &mut dyn ArgumentSource) -> Result<Self>;
}
