use crate::{Error, FlagArgument, FlagClass, Result};
use std::ffi::OsString;
use stream::ArgumentStream;

enum FlagArgumentResult {
    Handled,
    Help,
    NotFlag(OsString),
}

/// An object which parses command line arguments
pub struct Parser<'a, Options: 'a> {
    /// Program name to display for help
    name: Option<&'a dyn std::fmt::Display>,

    /// Program description displayed in the help
    description: Option<&'a dyn std::fmt::Display>,

    /// The usage description of the program
    ///
    /// The specified string can have special values:
    ///  * "%t" - Generates the terminal usage
    ///  * "%N" - Where "N" can be any number, generates the given command list value
    ///  * "%c" - Displays all values in the command list seperated by spaces
    ///
    /// A backslash preceding any character will print out just the character and won't process it
    /// for special values.
    usage: Option<&'a str>,

    /// Prologue for help
    prologue: Option<&'a dyn std::fmt::Display>,

    /// The header for non-grouped flags
    flag_header: Option<&'a dyn std::fmt::Display>,

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

mod help;
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
            usage: None,
            prologue: None,
            flag_header: None,
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
    pub const fn name<S: std::fmt::Display>(mut self, name: &'a S) -> Self {
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
    pub const fn description<S: std::fmt::Display>(mut self, description: &'a S) -> Self {
        self.description = Some(description);
        self
    }

    /// Sets the usage description of the program
    ///
    /// The specified string can have special values:
    ///  * "%t" - Generates the terminal usage
    ///  * "%N" - Where "N" can be any number, generates the given command list value
    ///  * "%c" - Displays all values in the command list seperated by spaces
    ///
    /// A backslash preceding any character will print out just the character and won't process it
    /// for special values.
    pub const fn usage(mut self, usage: &'a str) -> Self {
        self.usage = Some(usage);
        self
    }

    /// Sets the prologue displayed in the help
    ///
    /// ## Parameters
    ///  * `prologue` - The prologue that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified prologue
    pub const fn prologue<S: std::fmt::Display>(mut self, prologue: &'a S) -> Self {
        self.prologue = Some(prologue);
        self
    }

    /// Sets the header for non-grouped flags
    pub const fn flag_header<S: std::fmt::Display>(mut self, flag_header: &'a S) -> Self {
        self.flag_header = Some(flag_header);
        self
    }

    /// Sets the epilogue displayed in the help
    ///
    /// ## Parameters
    ///  * `epilogue` - The epilogue that will be displayed
    ///
    /// ## Return Value
    /// Returns this [`Parser`] with the modified epilogue
    pub const fn epilogue<S: std::fmt::Display>(mut self, epilogue: &'a S) -> Self {
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
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse<I: IntoIterator<Item = String>>(
        &self,
        options: Options,
        arguments: I,
        prefix_argument: Option<OsString>,
    ) -> Result<Option<Options>> {
        self.do_parse(
            options,
            &mut ArgumentStream::new(&mut arguments.into_iter()),
            prefix_argument
                .map(|prefix_argument| vec![prefix_argument])
                .unwrap_or(Vec::new()),
        )
    }

    /// Parses arguments from an iterator of [`OsString`]s
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `arguments` - The list of arguments to be parsed
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse_os<I: IntoIterator<Item = OsString>>(
        &self,
        options: Options,
        arguments: I,
        prefix_argument: Option<OsString>,
    ) -> Result<Option<Options>> {
        self.do_parse(
            options,
            &mut ArgumentStream::new_os(&mut arguments.into_iter()),
            prefix_argument
                .map(|prefix_argument| vec![prefix_argument])
                .unwrap_or(Vec::new()),
        )
    }

    /// Parses arguments using the environment
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse_env(&self, options: Options) -> Result<Option<Options>> {
        let mut args = std::env::args_os();
        let prefix = args.next().unwrap();

        self.parse_os(options, args, Some(prefix))
    }

    /// Parse arguments from an [`ArgumentStream`]
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `stream` - The stream of arguments to be parsed
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    fn do_parse(
        &self,
        mut options: Options,
        stream: &mut ArgumentStream,
        command_list: Vec<OsString>,
    ) -> Result<Option<Options>> {
        // Mark all flags as not ran
        let mut flags_ran = vec![false; self.flags.len()];

        while let Some(argument) = stream.next_os() {
            let argument = match self.handle_flag_argument(
                argument,
                &mut options,
                stream,
                &mut flags_ran,
                &command_list,
            )? {
                FlagArgumentResult::NotFlag(argument) => argument,
                FlagArgumentResult::Handled => continue,
                FlagArgumentResult::Help => return Ok(None),
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
        Ok(Some(options))
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
        command_list: &[OsString],
    ) -> Result<FlagArgumentResult> {
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
                help::generate(
                    self.name,
                    self.description,
                    self.usage,
                    command_list,
                    self.prologue,
                    self.flag_header,
                    self.flags,
                    self.epilogue,
                    self.short_prefix,
                    self.long_prefix,
                );

                if flag_argument.class() == FlagClass::Help {
                    std::process::exit(0);
                } else {
                    return Ok(FlagArgumentResult::Help);
                }
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

            Ok(FlagArgumentResult::Handled)
        } else {
            Ok(FlagArgumentResult::NotFlag(argument))
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
