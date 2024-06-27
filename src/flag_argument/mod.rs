use crate::{Error, Result};
use std::ffi::OsString;

/// The class of flag that a flag argument is
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagClass {
    /// A normal flag with a normal action
    Normal,

    /// A flag that should stop parsing immediately and return no result
    Interrupt,

    /// A flag that should display help and exit
    Help,

    /// A flag that should display help and stop parsing
    HelpNoExit,

    /// This flag reads a configuration file
    Config,
}

mod config;
mod help;
mod parsing;
mod simple;
mod version;

pub use config::ConfigFlagArgument;
pub use help::HelpFlagArgument;
pub use parsing::ParsingFlagArgument;
pub use simple::SimpleFlagArgument;
pub use version::VersionFlagArgument;

/// An argument triggered by a flag in the stream
pub trait FlagArgument<'a, Options: 'a> {
    /// Gets the short name for this flag
    ///
    /// ## Return Value
    /// Returns the short name for this flag if it has one
    fn short_name(&self) -> Option<&str>;

    /// Gets the long name for this flag
    ///
    /// ## Return Value
    /// Returns the long name for this flag if it has one
    fn long_name(&self) -> Option<&str>;

    /// Gets the maximum number of parameters to pull from the stream
    ///
    /// ## Return Value
    /// Returns the maximum number of parameters this flag requires from the argument stream
    fn count(&self) -> usize;

    /// Is this flag repeatable?
    ///
    /// ## Return Value
    /// Returns `true` if this flag can appear multiple times
    fn repeatable(&self) -> bool {
        false
    }

    /// The action called upon matching this flag
    ///
    /// ## Parameters
    ///  * `options` - The developer provided options that can be modified by this action
    ///  * `parameters` - The parameters of length at most the amount returned by `count`, may be
    ///                   less.
    ///
    /// ## Return Value
    /// Can return an error if there is a problem with the parameters
    fn action(&self, options: &mut Options, parameters: Vec<String>) -> Result<'a, ()>;

    /// The action called upon matching this flag
    ///
    /// ## Parameters
    ///  * `options` - The developer provided options that can be modified by this action
    ///  * `parameters` - The parameters of length at most the amount returned by `count`, may be
    ///                   less.
    ///
    /// ## Return Value
    /// Can return an error if there is a problem with the parameters
    fn action_os(&self, options: &mut Options, parameters: Vec<OsString>) -> Result<'a, ()> {
        self.action(
            options,
            parameters
                .into_iter()
                .map(|parameter| parameter.into_string().map_err(|_| Error::invalid_utf8()))
                .collect::<Result<_>>()?,
        )
    }

    /// Called for every flag in the current parsing upon completion of the parsing
    ///
    /// ## Parameters
    ///  * `ran` - `true` if this flag argument was run at least once, `false` otherwise
    ///
    /// ## Return Value
    /// Returns a result if there is an error
    ///
    /// ## Remarks
    /// An example usage of this function is to allow a flag argument to error if it is required
    /// but wasn't passed.
    #[allow(unused_variables)]
    fn finalize(&self, ran: bool) -> Result<()> {
        Ok(())
    }

    /// Gets the flag group this flag belongs in, if it is in one
    fn group(&self) -> Option<&str>;

    /// Gets the hint for displaying in the help
    ///
    /// ## Return Value
    /// Returns the hint if it has one
    fn hint(&self) -> Option<&dyn std::fmt::Display>;

    /// Gets the description for displaying the help
    ///
    /// ## Return Value
    /// Returns the description if it has one
    ///
    /// ## Remarks
    /// This function allows multi-line help descriptions by providing a slice return. Each element
    /// in the slice will be one line.
    fn description(&self) -> Option<&[&dyn std::fmt::Display]>;

    /// Gets the class of flag
    ///
    /// ## Return Value
    /// Returns the class of flag
    ///
    /// ## Remarks
    /// If this function returns `FlagClass::Help` or `FlagClass::HelpNoExit`, the parser ignores
    /// parsing parameters and running the action but instead runs the help generator to print out the help to `stdout`.
    fn class(&self) -> FlagClass {
        FlagClass::Normal
    }
}

impl<'a, Options: 'a, T: FlagArgument<'a, Options>> FlagArgument<'a, Options> for &T {
    fn short_name(&self) -> Option<&str> {
        T::short_name(self)
    }

    fn long_name(&self) -> Option<&str> {
        T::long_name(self)
    }

    fn count(&self) -> usize {
        T::count(self)
    }

    fn repeatable(&self) -> bool {
        T::repeatable(self)
    }

    fn action(&self, options: &mut Options, parameters: Vec<String>) -> Result<'a, ()> {
        T::action(self, options, parameters)
    }

    fn action_os(&self, options: &mut Options, parameters: Vec<OsString>) -> Result<'a, ()> {
        T::action_os(self, options, parameters)
    }

    fn finalize(&self, ran: bool) -> Result<()> {
        T::finalize(self, ran)
    }

    fn group(&self) -> Option<&str> {
        T::group(self)
    }

    fn hint(&self) -> Option<&dyn std::fmt::Display> {
        T::hint(self)
    }

    fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
        T::description(self)
    }

    fn class(&self) -> FlagClass {
        T::class(self)
    }
}

impl FlagClass {
    /// Should this flag display the help instead of running the action?
    ///
    /// ## Return Value
    /// Returns `true` if this flag should display the help
    pub const fn is_help(&self) -> bool {
        match self {
            FlagClass::Normal | FlagClass::Interrupt | FlagClass::Config => false,
            FlagClass::Help | FlagClass::HelpNoExit => true,
        }
    }

    /// Should this flag exit after running?
    ///
    /// ## Return Value
    /// Returns `true` if this flag should exit after running
    pub const fn is_exit(&self) -> bool {
        match self {
            FlagClass::Normal
            | FlagClass::HelpNoExit
            | FlagClass::Interrupt
            | FlagClass::Config => false,
            FlagClass::Help => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Error, FlagArgument, FlagClass, Result};
    use std::ffi::OsString;

    #[test]
    fn flag_class_is_help() {
        assert!(!FlagClass::Normal.is_help());
        assert!(!FlagClass::Interrupt.is_help());
        assert!(FlagClass::Help.is_help());
        assert!(FlagClass::HelpNoExit.is_help());
    }

    #[test]
    fn flag_class_is_exit() {
        assert!(!FlagClass::Normal.is_exit());
        assert!(!FlagClass::Interrupt.is_exit());
        assert!(FlagClass::Help.is_exit());
        assert!(!FlagClass::HelpNoExit.is_exit());
    }

    #[test]
    fn flag_argument_ref() {
        const SHORT_NAME: Option<&str> = Some("t");
        const LONG_NAME: Option<&str> = Some("test");
        const COUNT: usize = 1;
        const GROUP: Option<&str> = Some("EXAMPLE");
        const HINT: Option<&dyn std::fmt::Display> = Some(&"VAL");
        const DESCRIPTION: Option<&[&dyn std::fmt::Display]> = Some(&[&"Example"]);

        struct ExampleFlagArgument;

        impl<'a, Options: 'a> FlagArgument<'a, Options> for ExampleFlagArgument {
            fn short_name(&self) -> Option<&str> {
                SHORT_NAME
            }

            fn long_name(&self) -> Option<&str> {
                LONG_NAME
            }

            fn count(&self) -> usize {
                COUNT
            }

            fn action(&self, _: &mut Options, parameters: Vec<String>) -> Result<'a, ()> {
                if parameters.len() == COUNT {
                    Ok(())
                } else {
                    Err(Error::custom(""))
                }
            }

            fn action_os(&self, _: &mut Options, _: Vec<OsString>) -> Result<'a, ()> {
                panic!("Should not be called")
            }

            fn group(&self) -> Option<&str> {
                GROUP
            }

            fn hint(&self) -> Option<&dyn std::fmt::Display> {
                HINT
            }

            fn description(&self) -> Option<&[&dyn std::fmt::Display]> {
                DESCRIPTION
            }
        }

        let example = ExampleFlagArgument;
        let example_ref = &example;

        assert_eq!(FlagArgument::<()>::short_name(&example_ref), SHORT_NAME);
        assert_eq!(FlagArgument::<()>::long_name(&example_ref), LONG_NAME);
        assert_eq!(FlagArgument::<()>::group(&example_ref), GROUP);
        assert_eq!(FlagArgument::<()>::repeatable(&example_ref), false);
        assert!(FlagArgument::<()>::hint(&example_ref).is_some());
        assert!(FlagArgument::<()>::description(&example_ref).is_some());
        assert_eq!(FlagArgument::<()>::class(&example_ref), FlagClass::Normal);
        assert_eq!(FlagArgument::<()>::count(&example_ref), 1);

        assert!(FlagArgument::<()>::action(&example_ref, &mut (), Vec::new()).is_err());
        assert!(FlagArgument::<()>::action(&example_ref, &mut (), vec!["abc".to_owned()]).is_ok());

        assert!(FlagArgument::<()>::finalize(&example_ref, false).is_ok());
        assert!(FlagArgument::<()>::finalize(&example_ref, true).is_ok());
    }
}
