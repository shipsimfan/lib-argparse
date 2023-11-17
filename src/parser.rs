use crate::{FlagArgument, Result};
use std::ffi::OsString;

/// An object which parses command line arguments
pub struct Parser<Options: 'static> {
    /// Program name to display for help
    name: Option<&'static str>,

    /// Program description displayed in the help
    description: Option<&'static str>,

    /// Prologue for help
    prologue: Option<&'static str>,

    /// Epilogue for help
    epilogue: Option<&'static str>,

    /// Prefix for the short name of flag arguments
    short_prefix: &'static str,

    /// Prefix for the long name of flag arguments
    long_prefix: &'static str,

    /// The list of flag arguments
    flags: &'static [&'static dyn FlagArgument<Options>],
}

const DEFAULT_SHORT_PREFIX: &str = "-";
const DEFAULT_LONG_PREFIX: &str = "--";

impl<Options> Parser<Options> {
    /// Creates a new [`Parser`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Parser`]
    pub const fn new() -> Self {
        Parser {
            name: None,
            description: None,
            prologue: None,
            epilogue: None,
            short_prefix: DEFAULT_SHORT_PREFIX,
            long_prefix: DEFAULT_LONG_PREFIX,
            flags: &[],
        }
    }

    /// Sets the name displayed in the help
    ///
    /// ## Parameters
    ///  * `name` - The name that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified name
    pub const fn name(mut self, name: &'static str) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets the description displayed in the help
    ///
    /// ## Parameters
    ///  * `description` - The description that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified description
    pub const fn description(mut self, description: &'static str) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the prologue displayed in the help
    ///
    /// ## Parameters
    ///  * `prologue` - The prologue that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified prologue
    pub const fn prologue(mut self, prologue: &'static str) -> Self {
        self.prologue = Some(prologue);
        self
    }

    /// Sets the epilogue displayed in the help
    ///
    /// ## Parameters
    ///  * `epilogue` - The epilogue that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified epilogue
    pub const fn epilogue(mut self, epilogue: &'static str) -> Self {
        self.epilogue = Some(epilogue);
        self
    }

    /// Sets the short prefix for flag arguments
    ///
    /// ## Parameters
    ///  * `short_prefix` - The short prefix that will be used
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified short prefix
    pub const fn short_prefix(mut self, short_prefix: &'static str) -> Self {
        self.short_prefix = short_prefix;
        self
    }

    /// Sets the long prefix for flag arguments
    ///
    /// ## Parameters
    ///  * `long_prefix` - The long prefix that will be used
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified long prefix
    pub const fn long_prefix(mut self, long_prefix: &'static str) -> Self {
        self.long_prefix = long_prefix;
        self
    }

    /// Sets the flag arguments
    ///
    /// ## Parameters
    ///  * `flags` - The list of flag arguments
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the provided flags
    pub const fn flags(mut self, flags: &'static [&'static dyn FlagArgument<Options>]) -> Self {
        self.flags = flags;
        self
    }

    /// Parses arguments from an iterator of [`String`]s
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `arguments` - The list of arguments to be parsed
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful, returns the error otherwise.
    pub fn parse<I: IntoIterator<Item = String>>(
        &mut self,
        options: Options,
        arguments: I,
    ) -> Result<Options> {
        todo!("Implement `Parser::parse()`")
    }

    /// Parses arguments from an iterator of [`OsString`]s
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `arguments` - The list of arguments to be parsed
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful, returns the error otherwise.
    pub fn parse_os<I: IntoIterator<Item = OsString>>(
        &mut self,
        options: Options,
        arguments: I,
    ) -> Result<Options> {
        todo!("Implement `Parser::parse_os()`")
    }

    /// Parses arguments using the environment
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful, returns the error otherwise.
    pub fn parse_env(&mut self, options: Options) -> Result<Options> {
        self.parse_os(options, std::env::args_os())
    }
}
