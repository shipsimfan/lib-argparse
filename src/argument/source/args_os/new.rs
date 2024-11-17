use crate::ArgsOsSource;

impl<'a> ArgsOsSource<'a> {
    /// Creates a new [`ArgsOsSource`] from the user provided command line arguments
    pub fn new() -> Self {
        let mut args = std::env::args_os();
        let first = args.next().unwrap().into();

        ArgsOsSource { args, first }
    }
}
