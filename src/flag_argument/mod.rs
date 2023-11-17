use crate::Result;
use std::ffi::OsString;

/// The class of flag that a flag argument is
pub enum FlagClass {
    /// A normal flag with a normal action
    Normal,

    /// A flag that should display help and exit
    Help,

    /// A flag that should display help and stop parsing
    HelpNoExit,
}

/// An argument triggered by a flag in the stream
pub trait FlagArgument<Options: 'static> {
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
    fn action(&self, options: &mut Options, parameters: Vec<OsString>) -> Result<()>;

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

    /// Gets the hint for displaying in the help
    ///
    /// ## Return Value
    /// Returns the hint if it has one
    fn hint(&self) -> Option<&str>;

    /// Gets the description for displaying the help
    ///
    /// ## Return Value
    /// Returns the description if it has one
    ///
    /// ## Remarks
    /// This function allows multi-line help descriptions by providing a slice return. Each element
    /// in the slice will be one line.
    fn description(&self) -> Option<&[&str]>;

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

impl FlagClass {
    /// Should this flag display the help instead of running the action?
    ///
    /// ## Return Value
    /// Returns `true` if this flag should display the help
    pub const fn is_help(&self) -> bool {
        match self {
            FlagClass::Normal => false,
            FlagClass::Help | FlagClass::HelpNoExit => true,
        }
    }

    /// Should this flag exit after running?
    ///
    /// ## Return Value
    /// Returns `true` if this flag should exit after running
    pub const fn is_exit(&self) -> bool {
        match self {
            FlagClass::Normal | FlagClass::HelpNoExit => false,
            FlagClass::Help => true,
        }
    }
}
