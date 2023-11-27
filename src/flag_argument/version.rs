use crate::{FlagArgument, FlagClass};

/// A flag which displays the programs version and exits
pub struct VersionFlagArgument<'a> {
    /// The name to follow the short prefix
    short_name: Option<&'a str>,

    /// The name to follow the long prefix
    long_name: Option<&'a str>,

    /// The string which displays the programs version
    version: &'a dyn std::fmt::Display,

    /// The flag group this flag belongs to
    group: Option<&'a str>,

    /// Should the program after displaying the version?
    exit: bool,
}

impl<'a> VersionFlagArgument<'a> {
    /// Creates a new [`VersionFlagArgument`]
    ///
    /// `version` does not include the program's version by default. The environment variable for
    /// the crate version is `CARGO_PKG_VERSION`.
    pub const fn new<T: std::fmt::Display>(version: &'a T) -> Self {
        VersionFlagArgument {
            short_name: None,
            long_name: None,
            version,
            group: None,
            exit: true,
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

    /// Sets the string that displays the programs version
    pub const fn prefix<T: std::fmt::Display>(mut self, prefix: &'a T) -> Self {
        self.version = prefix;
        self
    }

    /// Sets the flag group this flag is apart of
    pub const fn group(mut self, group: &'a str) -> Self {
        self.group = Some(group);
        self
    }

    /// Sets the program to exit after displaying the version
    pub const fn set_exit(mut self) -> Self {
        self.exit = true;
        self
    }

    /// Sets the program to not exit after displaying the version
    pub const fn set_no_exit(mut self) -> Self {
        self.exit = false;
        self
    }
}

impl<'a, Options: 'a> FlagArgument<'a, Options> for VersionFlagArgument<'a> {
    fn short_name(&self) -> Option<&str> {
        self.short_name
    }

    fn long_name(&self) -> Option<&str> {
        self.long_name
    }

    fn count(&self) -> usize {
        0
    }

    fn action(&self, _: &mut Options, _: Vec<std::ffi::OsString>) -> crate::Result<()> {
        println!("{}", self.version);

        if self.exit {
            std::process::exit(0);
        }

        Ok(())
    }

    fn repeatable(&self) -> bool {
        true
    }

    fn group(&self) -> Option<&str> {
        None
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        None
    }

    fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
        Some(&[&"Displays this programs version"])
    }

    fn class(&self) -> FlagClass {
        FlagClass::Interrupt
    }
}
