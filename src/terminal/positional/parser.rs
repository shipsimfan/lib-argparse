use std::ffi::OsString;

pub trait PositionalParser {
    type Options;
    type Error;

    /// Parses the positional arguments
    ///
    /// Returning `true` means this parser will take the next positional
    ///
    /// Returning `false` means this parser is done
    fn parse(&mut self, options: &mut Self::Options, arg: OsString) -> Result<bool, Self::Error>;

    /// Called at the end of argument parsing
    ///
    /// Example usage: A parser returning an error if not enough arguments were passed
    #[allow(unused_variables)]
    fn finish(&mut self, options: &mut Self::Options) -> Result<(), Self::Error> {
        Ok(())
    }
}
