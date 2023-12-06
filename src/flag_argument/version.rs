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
    pub const fn version<T: std::fmt::Display>(mut self, version: &'a T) -> Self {
        self.version = version;
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
        self.group
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

#[cfg(test)]
mod tests {
    use crate::{FlagArgument, FlagClass, VersionFlagArgument};

    #[test]
    fn create_default() {
        const SHORT_NAME: &str = "t";
        const LONG_NAME: &str = "test";
        const GROUP: &str = "EXAMPLE";
        const VERSION: &str = "example v1";

        let mut version_flag = VersionFlagArgument::new(&"").set_no_exit();

        assert_eq!(FlagArgument::<()>::short_name(&version_flag), None);
        assert_eq!(FlagArgument::<()>::long_name(&version_flag), None);
        assert_eq!(FlagArgument::<()>::group(&version_flag), None);
        assert_eq!(
            FlagArgument::<()>::class(&version_flag),
            FlagClass::Interrupt
        );

        assert_eq!(FlagArgument::<()>::count(&version_flag), 0);
        assert_eq!(FlagArgument::<()>::repeatable(&version_flag), true);
        assert!(FlagArgument::<()>::action(&version_flag, &mut (), Vec::new()).is_ok());
        assert!(FlagArgument::<()>::hint(&version_flag).is_none());
        assert!(FlagArgument::<()>::description(&version_flag).is_some());

        version_flag = version_flag
            .short_name(SHORT_NAME)
            .long_name(LONG_NAME)
            .group(GROUP)
            .version(&VERSION);

        assert_eq!(
            FlagArgument::<()>::short_name(&version_flag),
            Some(SHORT_NAME)
        );
        assert_eq!(
            FlagArgument::<()>::long_name(&version_flag),
            Some(LONG_NAME)
        );
        assert_eq!(FlagArgument::<()>::group(&version_flag), Some(GROUP));
        assert_eq!(
            FlagArgument::<()>::class(&version_flag),
            FlagClass::Interrupt
        );

        version_flag = version_flag.set_exit();

        assert_eq!(
            FlagArgument::<()>::class(&version_flag),
            FlagClass::Interrupt
        );
    }
}
