use crate::{Error, FlagArgument, Result};
use std::ffi::OsString;
use stream::ArgumentStream;

/// An object which parses command line arguments
pub struct Parser<'a, Options: 'a> {
    /// Program name to display for help
    name: Option<&'a dyn std::fmt::Display>,

    /// Program description displayed in the help
    description: Option<&'a dyn std::fmt::Display>,

    /// Prologue for help
    prologue: Option<&'a dyn std::fmt::Display>,

    /// Epilogue for help
    epilogue: Option<&'a dyn std::fmt::Display>,

    /// Prefix for the short name of flag arguments
    short_prefix: &'a str,

    /// Prefix for the long name of flag arguments
    long_prefix: &'a str,

    /// The list of flag arguments
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
}

const DEFAULT_SHORT_PREFIX: &str = "-";
const DEFAULT_LONG_PREFIX: &str = "--";

mod stream;

impl<'a, Options> Parser<'a, Options> {
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
    pub const fn name(mut self, name: &'a dyn std::fmt::Display) -> Self {
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
    pub const fn description(mut self, description: &'a dyn std::fmt::Display) -> Self {
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
    pub const fn prologue(mut self, prologue: &'a dyn std::fmt::Display) -> Self {
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
    pub const fn epilogue(mut self, epilogue: &'a dyn std::fmt::Display) -> Self {
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
    pub const fn short_prefix(mut self, short_prefix: &'a str) -> Self {
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
    pub const fn long_prefix(mut self, long_prefix: &'a str) -> Self {
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
    pub const fn flags(mut self, flags: &'a [&'a dyn FlagArgument<'a, Options>]) -> Self {
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
        &self,
        options: Options,
        arguments: I,
    ) -> Result<Options> {
        self.do_parse(
            options,
            &mut ArgumentStream::new(&mut arguments.into_iter()),
        )
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
        &self,
        options: Options,
        arguments: I,
    ) -> Result<Options> {
        self.do_parse(
            options,
            &mut ArgumentStream::new_os(&mut arguments.into_iter()),
        )
    }

    /// Parses arguments using the environment
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful, returns the error otherwise.
    pub fn parse_env(&self, options: Options) -> Result<Options> {
        self.parse_os(options, std::env::args_os())
    }

    /// Parse arguments from an [`ArgumentStream`]
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `stream` - The stream of arguments to be parsed
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful, returns the error otherwise.
    fn do_parse(&self, mut options: Options, stream: &mut ArgumentStream) -> Result<Options> {
        // Remove the zero argument
        let zero_argument = stream.next_os().ok_or_else(Error::missing_zero_argument)?;

        // Mark all flags as not ran
        let mut flags_ran = vec![false; self.flags.len()];

        while let Some(argument) = stream.next_os() {
            let argument =
                match self.handle_flag_argument(argument, &mut options, stream, &mut flags_ran)? {
                    Some(argument) => argument,
                    None => continue,
                };

            return Err(Error::unexpected_argument(format!(
                "unexpected argument \"{}\"",
                argument.to_string_lossy()
            )));
        }

        // Finalize flags
        for (i, flag) in self.flags.into_iter().enumerate() {
            flag.finalize(flags_ran[i])?;
        }

        // Return options
        Ok(options)
    }

    /// Handles a flag argument during parsing
    ///
    /// ## Parameters
    ///  * `argument` - The argument to be handled
    ///  * `options` - The options to modified by the parser
    ///  * `stream` - The stream of arguments to be parsed from
    ///  * `flags_ran` - The slice of flag running information
    ///
    /// ## Return Value
    /// Returns the argument if it is not a flag argument. Returns `Ok(None)` on successful parsing
    /// of the flag argument or the error if it is unsuccessful.
    fn handle_flag_argument(
        &self,
        argument: OsString,
        options: &mut Options,
        stream: &mut ArgumentStream,
        flags_ran: &mut [bool],
    ) -> Result<Option<OsString>> {
        // Check for long or short prefix
        let is_long = argument
            .as_encoded_bytes()
            .starts_with(self.long_prefix.as_bytes());
        let is_short = argument
            .as_encoded_bytes()
            .starts_with(self.short_prefix.as_bytes())
            && !is_long;

        if is_long || is_short {
            // Convert to UTF-8
            let argument = argument.into_string().map_err(Into::<Error>::into)?;

            // Find the flag argument
            let (flag_argument, flag_index) = self
                .get_flag_argument(
                    if is_short {
                        Some(&argument[self.short_prefix.len()..])
                    } else {
                        None
                    },
                    if is_long {
                        Some(&argument[self.long_prefix.len()..])
                    } else {
                        None
                    },
                )
                .ok_or_else(|| Error::unknown_flag(format!("unknown flag \"{}\"", argument)))?;

            // Make sure its not repeated
            if !flag_argument.repeatable() && flags_ran[flag_index] {
                return Err(Error::repeated_flag(format!(
                    "\"{}\" cannot appear more than once",
                    argument
                )));
            }
            flags_ran[flag_index] = true;

            // Check if the flag is a help flag
            if flag_argument.class().is_help() {
                todo!("Implement help generator");
            }

            // Parse the parameters from the stream
            let count = flag_argument.count();
            let mut parameters = Vec::with_capacity(count);
            for _ in 0..count {
                match stream.next_os() {
                    Some(parameter) => parameters.push(parameter),
                    None => break,
                }
            }

            // Call the action
            flag_argument.action(options, parameters)?;

            Ok(None)
        } else {
            Ok(Some(argument))
        }
    }

    /// Gets the flag argument given its name
    ///
    /// ## Parameters
    ///  * `long_name` - The long name to look for
    ///  * `short_name` - The short name to look for
    ///
    /// ## Return Value
    /// If found, it will return the flag argument and its index
    fn get_flag_argument(
        &self,
        short_name: Option<&str>,
        long_name: Option<&str>,
    ) -> Option<(&'a dyn FlagArgument<'a, Options>, usize)> {
        assert!(short_name.is_some() || long_name.is_some());

        for (i, flag) in self.flags.iter().enumerate() {
            if short_name.is_some() && flag.short_name() == short_name {
                return Some((*flag, i));
            }

            if long_name.is_some() && flag.long_name() == long_name {
                return Some((*flag, i));
            }
        }

        None
    }
}
