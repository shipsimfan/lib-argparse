use crate::{terminal_argument::TerminalArgument, Error, FlagArgument, FlagClass, Result};
use help::StdOut;
use io::IOArgumentParser;
use std::{ffi::OsString, fs::File, io::Read, path::PathBuf};
use stream::ArgumentStream;

mod help;
mod io;
mod stream;

enum FlagArgumentResult {
    Handled,
    Help,
    Read(PathBuf),
    NotFlag(OsString),
}

/// An object which parses command line arguments
pub struct Parser<'a, Options: 'a> {
    /// Program name to display for help
    name: Option<&'a dyn std::fmt::Display>,

    /// Program description displayed in the help
    description: &'a [&'a dyn std::fmt::Display],

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

    /// The terminal argument
    terminal: Option<&'a dyn TerminalArgument<'a, Options>>,
}

const DEFAULT_SHORT_PREFIX: &str = "-";
const DEFAULT_LONG_PREFIX: &str = "--";

impl<'a, Options> Parser<'a, Options> {
    /// Creates a new [`Parser`]
    ///
    /// ## Return Value
    /// Returns the newly created [`Parser`]
    pub const fn new() -> Self {
        Parser {
            name: None,
            description: &[],
            usage: None,
            prologue: None,
            flag_header: None,
            epilogue: None,
            short_prefix: DEFAULT_SHORT_PREFIX,
            long_prefix: DEFAULT_LONG_PREFIX,
            flags: &[],
            terminal: None,
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
    pub const fn description(mut self, description: &'a [&'a dyn std::fmt::Display]) -> Self {
        self.description = description;
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

    /// Sets the terminal argument
    pub const fn terminal(mut self, terminal: &'a dyn TerminalArgument<'a, Options>) -> Self {
        self.terminal = Some(terminal);
        self
    }
    /// Parses arguments from an iterator of [`String`]s
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `arguments` - The list of arguments to be parsed
    ///  * `prefix_argument` - The argument to be provided to the usage displayed in a help message
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse<I: IntoIterator<Item = String>>(
        &self,
        options: Options,
        arguments: I,
        prefix_argument: Option<OsString>,
    ) -> Result<'a, Option<Options>> {
        self.do_parse(
            options,
            &mut ArgumentStream::new(&mut arguments.into_iter()),
            &mut prefix_argument
                .map(|prefix_argument| vec![prefix_argument])
                .unwrap_or(Vec::new()),
            self.long_prefix.as_bytes(),
            self.short_prefix.as_bytes(),
        )
    }

    /// Parses arguments from an iterator of [`OsString`]s
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `arguments` - The list of arguments to be parsed
    ///  * `prefix_argument` - The argument to be provided to the usage displayed in a help message
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse_os<I: IntoIterator<Item = OsString>>(
        &self,
        options: Options,
        arguments: I,
        prefix_argument: Option<OsString>,
    ) -> Result<'a, Option<Options>> {
        self.do_parse(
            options,
            &mut ArgumentStream::new_os(&mut arguments.into_iter()),
            &mut prefix_argument
                .map(|prefix_argument| vec![prefix_argument])
                .unwrap_or(Vec::new()),
            self.long_prefix.as_bytes(),
            self.short_prefix.as_bytes(),
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
    pub fn parse_env(&self, options: Options) -> Result<'a, Option<Options>> {
        let mut args = std::env::args_os();
        let prefix = args.next().unwrap();

        self.parse_os(options, args, Some(prefix))
    }

    /// Parses arguments from `source`
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `source` - The source of arguments to parse
    ///  * `prefix_argument` - The argument to be provided to the usage displayed in a help message
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    pub fn parse_read(
        &self,
        options: Options,
        source: &mut dyn Read,
        prefix_argument: Option<OsString>,
    ) -> Result<'a, Option<Options>> {
        self.do_parse(
            options,
            &mut ArgumentStream::IO(IOArgumentParser::new(source)),
            &mut prefix_argument
                .map(|prefix_argument| vec![prefix_argument])
                .unwrap_or(Vec::new()),
            self.short_prefix.as_bytes(),
            self.long_prefix.as_bytes(),
        )
    }

    /// Parse arguments from an [`ArgumentStream`]
    ///
    /// ## Parameters
    ///  * `options` - The options to modified by the parser
    ///  * `stream` - The stream of arguments to be parsed
    ///  * `command_list` - The list of commands that have been seen already
    ///
    /// ## Return Value
    /// Returns the changed options if parsing is successful and no help flag was matched, returns
    /// the error otherwise.
    fn do_parse(
        &self,
        mut options: Options,
        stream: &mut ArgumentStream,
        command_list: &mut Vec<OsString>,
        long_prefix: &[u8],
        short_prefix: &[u8],
    ) -> Result<'a, Option<Options>> {
        // Mark all flags as not ran
        let mut flags_ran = vec![false; self.flags.len()];
        let mut terminal_index = 0;

        while let Some(argument) = stream.next_os()? {
            let argument = match self.handle_flag_argument(
                argument,
                &mut options,
                stream,
                &mut flags_ran,
                &command_list,
                self.terminal,
                long_prefix,
                short_prefix,
            )? {
                FlagArgumentResult::NotFlag(argument) => argument,
                FlagArgumentResult::Read(path) => {
                    assert!(self.terminal.is_none());

                    let mut file = File::open(&path).map_err(|error| {
                        Error::io(format!("unable to read \"{}\" - {}", path.display(), error))
                    })?;

                    match self.do_parse(
                        options,
                        &mut ArgumentStream::IO(IOArgumentParser::new(&mut file)),
                        command_list,
                        b"",
                        b"",
                    )? {
                        Some(new_options) => {
                            options = new_options;
                            continue;
                        }
                        None => return Ok(None),
                    }
                }
                FlagArgumentResult::Handled => continue,
                FlagArgumentResult::Help => return Ok(None),
            };

            if let Some(terminal) = self.terminal {
                if let Some(parser) =
                    terminal.action(&mut options, terminal_index, argument.clone())?
                {
                    command_list.push(argument);
                    return parser.do_parse(
                        options,
                        stream,
                        command_list,
                        long_prefix,
                        short_prefix,
                    );
                }

                terminal_index += 1;
            } else {
                return Err(Error::unexpected_argument(format!(
                    "unexpected argument \"{}\"",
                    argument.to_string_lossy()
                )));
            }
        }

        // Finalize terminal
        if let Some(terminal) = self.terminal {
            terminal.finalize(terminal_index)?;
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
    fn handle_flag_argument<'b>(
        &self,
        argument: OsString,
        options: &mut Options,
        stream: &mut ArgumentStream,
        flags_ran: &mut [bool],
        command_list: &[OsString],
        terminal: Option<&dyn TerminalArgument<'a, Options>>,
        long_prefix: &[u8],
        short_prefix: &[u8],
    ) -> Result<'a, FlagArgumentResult> {
        // Check for long or short prefix
        let is_long = argument.as_encoded_bytes().starts_with(long_prefix);
        let is_short = argument.as_encoded_bytes().starts_with(short_prefix) && !is_long;

        if is_long || is_short {
            // Convert to UTF-8
            let argument = argument.into_string().map_err(Into::<Error>::into)?;

            // Find the flag argument
            let (flag_argument, flag_index) = self
                .get_flag_argument(
                    if is_short {
                        Some(&argument[short_prefix.len()..])
                    } else {
                        None
                    },
                    if is_long {
                        Some(&argument[long_prefix.len()..])
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
                    terminal,
                    &mut StdOut::new(),
                )
                .unwrap();

                if flag_argument.class() == FlagClass::Help {
                    std::process::exit(0);
                } else {
                    return Ok(FlagArgumentResult::Help);
                }
            }

            let count = flag_argument.count();
            let path = if stream.is_os() {
                // Parse the parameters from the stream as `OsString`s
                let mut parameters = Vec::with_capacity(count);
                for _ in 0..count {
                    match stream.next_os()? {
                        Some(parameter) => parameters.push(parameter),
                        None => break,
                    }
                }

                // Call the action
                if flag_argument.class() != FlagClass::Config {
                    flag_argument.action_os(options, parameters)?;
                    None
                } else {
                    assert_eq!(count, 1);
                    Some(PathBuf::from(parameters.pop().ok_or(
                        Error::missing_parameters("missing PATH for configuration flag"),
                    )?))
                }
            } else {
                // Parse the parameters from the stream as `String`s
                let mut parameters = Vec::with_capacity(count);
                for _ in 0..count {
                    match stream.next()? {
                        Some(parameter) => parameters.push(parameter),
                        None => break,
                    }
                }

                // Call the action
                if flag_argument.class() != FlagClass::Config {
                    flag_argument.action(options, parameters)?;
                    None
                } else {
                    assert_eq!(count, 1);
                    Some(PathBuf::from(parameters.pop().ok_or(
                        Error::missing_parameters("missing PATH for configuration flag"),
                    )?))
                }
            };

            Ok(match flag_argument.class() {
                FlagClass::Interrupt => FlagArgumentResult::Help,
                FlagClass::Config => FlagArgumentResult::Read(path.unwrap()),
                _ => FlagArgumentResult::Handled,
            })
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
