use crate::{FlagArgument, FlagClass, Result};

/// A flag which displays a help message
pub struct HelpFlagArgument<'a> {
    /// The name to follow the short prefix
    short_name: Option<&'a str>,

    /// The name to follow the long prefix
    long_name: Option<&'a str>,

    /// The flag group this flag belongs to
    group: Option<&'a str>,

    /// Should the program after displaying the help?
    exit: FlagClass,
}

impl<'a> HelpFlagArgument<'a> {
    /// Creates a new [`HelpFlag`]
    pub const fn new() -> Self {
        HelpFlagArgument {
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

impl<'a, Options: 'a> FlagArgument<'a, Options> for HelpFlagArgument<'a> {
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

#[cfg(test)]
mod tests {
    use crate::{FlagArgument, FlagClass, HelpFlagArgument};

    #[test]
    fn create() {
        const SHORT_NAME: &str = "t";
        const LONG_NAME: &str = "test";
        const GROUP: &str = "EXAMPLE";

        let mut help_flag = HelpFlagArgument::new();

        assert_eq!(FlagArgument::<()>::short_name(&help_flag), None);
        assert_eq!(FlagArgument::<()>::long_name(&help_flag), None);
        assert_eq!(FlagArgument::<()>::group(&help_flag), None);
        assert_eq!(FlagArgument::<()>::class(&help_flag), FlagClass::Help);

        assert_eq!(FlagArgument::<()>::count(&help_flag), 0);
        assert_eq!(FlagArgument::<()>::repeatable(&help_flag), true);
        assert!(FlagArgument::<()>::action(&help_flag, &mut (), Vec::new()).is_ok());
        assert!(FlagArgument::<()>::hint(&help_flag).is_none());
        assert!(FlagArgument::<()>::description(&help_flag).is_some());

        help_flag = help_flag
            .short_name(SHORT_NAME)
            .long_name(LONG_NAME)
            .group(GROUP)
            .set_no_exit();

        assert_eq!(FlagArgument::<()>::short_name(&help_flag), Some(SHORT_NAME));
        assert_eq!(FlagArgument::<()>::long_name(&help_flag), Some(LONG_NAME));
        assert_eq!(FlagArgument::<()>::group(&help_flag), Some(GROUP));
        assert_eq!(FlagArgument::<()>::class(&help_flag), FlagClass::HelpNoExit);

        help_flag = help_flag.set_exit();

        assert_eq!(FlagArgument::<()>::class(&help_flag), FlagClass::Help);
    }
}
