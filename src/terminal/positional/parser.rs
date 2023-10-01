use std::ffi::OsString;

/// A parser for positional arguments
pub trait PositionalParser {
    /// The options object to be modified
    type Options;

    /// An error returned if parsing failed
    type Error;

    /// Parses the positional arguments
    ///
    /// The return value determines if this parser will take the next positional:
    ///  - `true` means this parser will take the next positional
    ///  - `false` means this parser is done
    ///
    ///  - `options` is the developer provided options to update
    ///  - `argument` is the argument to be parsed from the stream
    fn parse(
        &mut self,
        options: &mut Self::Options,
        argument: OsString,
    ) -> Result<bool, crate::Error<Self::Error>>;

    /// Called at the end of argument parsing
    ///
    ///  - `options` is the developer provided options which can be updated
    ///
    /// Example usage: A parser returning an error if not enough arguments were passed
    #[allow(unused_variables)]
    fn finalize(&mut self, options: &mut Self::Options) -> Result<(), crate::Error<Self::Error>> {
        Ok(())
    }

    /// Generates the usage hint for help
    ///
    ///  - `f` is the output
    fn generate_usage(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}
