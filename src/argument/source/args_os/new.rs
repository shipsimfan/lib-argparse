use crate::ArgsOsSource;

impl ArgsOsSource {
    /// Creates a new [`ArgsOsSource`] from the user provided command line arguments
    pub fn new() -> Self {
        let args = std::env::args_os();

        ArgsOsSource { args }
    }
}
