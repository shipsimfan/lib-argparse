use crate::ArgsSource;

impl ArgsSource {
    /// Creates a new [`ArgsSource`] from the user provided command line arguments
    pub fn new() -> Self {
        let args = std::env::args();

        ArgsSource { args }
    }
}
