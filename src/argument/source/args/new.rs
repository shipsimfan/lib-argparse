use crate::ArgsSource;

impl<'a> ArgsSource<'a> {
    /// Creates a new [`ArgsSource`] from the user provided command line arguments
    pub fn new() -> Self {
        let mut args = std::env::args();
        let first = args.next().unwrap().into();

        ArgsSource { args, first }
    }
}
