use std::ffi::OsString;

/// A parser for positional arguments
pub trait PositionalParser {
    /// The options object to be modified
    type Options;

    /// An error returned if parsing failed
    type Error;

    /// Parses the positional arguments
    ///
    /// Returning `true` means this parser will take the next positional
    ///
    /// Returning `false` means this parser is done
    fn parse(
        &mut self,
        options: &mut Self::Options,
        arg: OsString,
    ) -> Result<bool, crate::Error<Self::Error>>;

    /// Called at the end of argument parsing
    ///
    /// Example usage: A parser returning an error if not enough arguments were passed
    #[allow(unused_variables)]
    fn finish(&mut self, options: &mut Self::Options) -> Result<(), crate::Error<Self::Error>> {
        Ok(())
    }
}
