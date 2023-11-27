use crate::{FlagArgument, FlagClass, Result};

/// A flag which displays a help message
pub struct HelpFlag<'a> {
    /// The name to follow the short prefix
    short_name: Option<&'a str>,

    /// The name to follow the long prefix
    long_name: Option<&'a str>,

    /// The flag group this flag belongs to
    group: Option<&'a str>,

    /// Should the program after displaying the help?
    exit: FlagClass,
}

impl<'a> HelpFlag<'a> {
    /// Creates a new [`HelpFlag`]
    pub const fn new() -> Self {
        HelpFlag {
            short_name: None,
            long_name: None,
            group: None,
            exit: FlagClass::Help,
        }
    }

    /// Sets the name which follows the short prefix
    pub const fn short_name(mut self, short_name: &'a str) -> Self {
        self.short_name = Some(short_name);
        self
    }

    /// Sets the name which follows the long prefix
    pub const fn long_name(mut self, long_name: &'a str) -> Self {
        self.long_name = Some(long_name);
        self
    }

    /// Sets the flag group this flag is apart of
    pub const fn group(mut self, group: &'a str) -> Self {
        self.group = Some(group);
        self
    }

    /// Sets the program to exit after displaying the help
    pub const fn set_exit(mut self) -> Self {
        self.exit = FlagClass::Help;
        self
    }

    /// Sets the program to not exit after displaying the help
    pub const fn set_no_exit(mut self) -> Self {
        self.exit = FlagClass::HelpNoExit;
        self
    }
}

impl<'a, Options: 'a> FlagArgument<'a, Options> for HelpFlag<'a> {
    fn short_name(&self) -> Option<&str> {
        self.short_name
    }

    fn long_name(&self) -> Option<&str> {
        self.long_name
    }

    fn count(&self) -> usize {
        0
    }

    fn repeatable(&self) -> bool {
        true
    }

    fn action(&self, _: &mut Options, _: Vec<std::ffi::OsString>) -> Result<()> {
        Ok(())
    }

    fn group(&self) -> Option<&str> {
        self.group
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        None
    }

    fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
        Some(&[&"Displays this help message"])
    }

    fn class(&self) -> FlagClass {
        self.exit
    }
}
