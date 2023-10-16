use crate::{ArgStream, Command, Error, FlagArgument, FlagSet, Positionals, TerminalArgument};
use help::HelpGenerator;
use std::{
    borrow::Cow,
    ffi::{OsStr, OsString},
};

mod help;

enum ParseResult<'a, T, E: 'static> {
    Continue(&'a mut Parser<T, E>, String),
    Complete,
    Help,
}

/// Argument Parser
pub struct Parser<T, E: 'static = ()> {
    /// Program name to display for help
    program_name: Option<Cow<'static, str>>,
    /// Program description for help
    description: Option<Cow<'static, str>>,

    /// Prologue for help
    prologue: Option<Cow<'static, str>>,
    /// Epilogue for help
    epilogue: Option<Cow<'static, str>>,

    /// Prefixes for flag arguments
    short_prefix: Cow<'static, str>,
    long_prefix: Cow<'static, str>,

    /// Arguments
    flag_arguments: FlagSet<T, E>,
    terminal_argument: TerminalArgument<T, E>,
}

/// Determines if an argument starts with a prefix
///
///  - `argument` is the argument to be checked
///  - `prefix` is the value to be checked against
fn starts_with(argument: &OsStr, prefix: &str) -> bool {
    if argument.len() < prefix.len() {
        return false;
    }

    &argument.as_encoded_bytes()[..prefix.len()] == prefix.as_bytes()
}

impl<T, E> Parser<T, E> {
    /// Creates a new empty [`Parser`]
    pub fn new() -> Self {
        Parser {
            program_name: None,
            description: None,

            prologue: None,
            epilogue: None,

            short_prefix: "-".into(),
            long_prefix: "--".into(),

            flag_arguments: FlagSet::new(),
            terminal_argument: TerminalArgument::None,
        }
    }

    /// Returns the program name
    pub fn program_name(&self) -> Option<&str> {
        self.program_name.as_deref()
    }

    /// Returns the program description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the help prologue
    pub fn prologue(&self) -> Option<&str> {
        self.prologue.as_deref()
    }

    /// Returns the help epilogue
    pub fn epilogue(&self) -> Option<&str> {
        self.epilogue.as_deref()
    }

    /// Returns the flag arugments
    pub fn flag_arguments(&self) -> &[FlagArgument<T, E>] {
        &self.flag_arguments
    }

    /// Returns the terminal argument
    pub fn terminal_argument(&self) -> &TerminalArgument<T, E> {
        &self.terminal_argument
    }

    /// Returns the prefix for short names
    pub fn short_prefix(&self) -> &str {
        &self.short_prefix
    }

    /// Returns the prefix for long names
    pub fn long_prefix(&self) -> &str {
        &self.long_prefix
    }

    /// Sets the program name
    ///
    /// If this parser is in a [`Command`], then the program name used will be inherited instead of the one set here.
    ///
    ///  - `program_name` is the string the program name will be set to
    pub fn set_program_name<S: Into<Cow<'static, str>>>(mut self, program_name: S) -> Self {
        self.program_name = Some(program_name.into());
        self
    }

    /// Sets the program description
    ///
    /// If this parser is in a [`Command`], then the program description used will be inherited instead of the one set here.
    ///
    ///  - `description` is the string the program description will be set to
    pub fn set_description<S: Into<Cow<'static, str>>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the help prologue
    ///
    ///  - `prologue` is the string the program description will be set to
    pub fn set_prologue<S: Into<Cow<'static, str>>>(mut self, prologue: S) -> Self {
        self.prologue = Some(prologue.into());
        self
    }

    /// Sets the help epilogue
    ///
    ///  - `epilogue` is the string the program description will be set to
    pub fn set_epilogue<S: Into<Cow<'static, str>>>(mut self, epilogue: S) -> Self {
        self.epilogue = Some(epilogue.into());
        self
    }

    /// Pushes an argument into the list of flag arguments
    ///
    /// Returns any arguments which have conflicting names.
    /// If there is only one conflict, the conflicting argument will always be the first value of the returned tuple.
    ///
    ///  - `flag_argument` is the argument to pushed into the list
    pub fn add_flag_argument(
        &mut self,
        flag_argument: FlagArgument<T, E>,
    ) -> (Option<FlagArgument<T, E>>, Option<FlagArgument<T, E>>) {
        self.flag_arguments.push(flag_argument)
    }

    /// Sets the terminal argument
    ///
    ///  - `terminal_argument` is the terminal argument the terminal argument will be set to
    pub fn set_terminal_argument(mut self, terminal_argument: TerminalArgument<T, E>) -> Self {
        self.terminal_argument = terminal_argument;
        self
    }

    /// Sets the terminal argument to a [`Command`]
    ///
    ///  - `command` is the [`Command`] the terminal argument will be set to
    pub fn set_terminal_command(mut self, command: Command<T, E>) -> Self {
        self.terminal_argument = TerminalArgument::Command(command);
        self
    }

    /// Sets the terminal arugment to a [`Positionals`]
    ///
    ///  - `positionals` is the [`Positionals`] the terminal argument will be set to
    pub fn set_terminal_positionals(mut self, positionals: Positionals<T, E>) -> Self {
        self.terminal_argument = TerminalArgument::Positionals(positionals);
        self
    }

    /// Removes the terminal argument
    pub fn clear_terminal_argument(mut self) -> Self {
        self.terminal_argument = TerminalArgument::None;
        self
    }

    /// Sets the prefix for short names
    ///
    ///  - `short_prefix` is the string the short prefix will be set to
    pub fn set_short_prefix<S: Into<Cow<'static, str>>>(mut self, short_prefix: S) -> Self {
        let short_prefix = short_prefix.into();
        assert_ne!(
            self.long_prefix, short_prefix,
            "Short prefix and long prefix cannot be the same"
        );
        self.short_prefix = short_prefix;
        self
    }

    /// Sets the prefix for long names
    ///
    ///  - `long_prefix` is the string the long prefix will be set to
    pub fn set_long_prefix<S: Into<Cow<'static, str>>>(mut self, long_prefix: S) -> Self {
        let long_prefix = long_prefix.into();
        assert_ne!(
            self.short_prefix, long_prefix,
            "Short prefix and long prefix cannot be the same"
        );
        self.long_prefix = long_prefix;
        self
    }

    /// Parses arguments from the [`String`] stream
    ///
    ///  - `iter` is the [`String`] iterator to be parsed from
    ///  - `options` is the developer provided options to be updated
    ///
    /// Returns if the parsing completed. `false` if this returned from a help flag or `true` if this completed.
    pub fn parse<I: IntoIterator<Item = String> + 'static>(
        &mut self,
        iter: I,
        options: &mut T,
    ) -> Result<bool, Error<E>> {
        self.begin_parse(ArgStream::new(iter), options)
    }

    /// Parses arguments from the [`OsString`] stream
    ///
    ///  - `iter` is the [`OsString`] iterator to be parsed from
    ///  - `options` is the developer provided options to be updated
    ///
    /// Returns if the parsing completed. `false` if this returned from a help flag or `true` if this completed.
    pub fn parse_os<I: IntoIterator<Item = OsString> + 'static>(
        &mut self,
        iter: I,
        options: &mut T,
    ) -> Result<bool, Error<E>> {
        self.begin_parse(ArgStream::new_os(iter), options)
    }

    /// Parses arguments from the environment
    ///
    ///  - `options` is the developer provided options to be updated
    ///
    /// Returns if the parsing completed. `false` if this returned from a help flag or `true` if this completed.
    pub fn parse_env(&mut self, options: &mut T) -> Result<bool, Error<E>> {
        self.begin_parse(ArgStream::new_env(), options)
    }

    /// Parses arguments from the stream
    ///
    ///  - `args` is the argument stream to be parse from
    ///  - `options` is the developer provided options to be updated
    ///
    /// Returns if the parsing completed. `false` if this returned from a help flag or `true` if this completed.
    fn begin_parse(&mut self, mut args: ArgStream, options: &mut T) -> Result<bool, Error<E>> {
        let first_argument = args.next()?.unwrap();

        let program_name = self.program_name().map(|string| string.to_owned());
        let description = self.description().map(|string| string.to_owned());

        let mut parser = self;
        let mut command_chain = Vec::new();
        loop {
            match parser.do_parse(
                options,
                &mut args,
                &command_chain,
                &first_argument,
                program_name.as_deref(),
                description.as_deref(),
            )? {
                ParseResult::Continue(new_parser, command) => {
                    parser = new_parser;
                    command_chain.push(command);
                }
                ParseResult::Complete => return Ok(true),
                ParseResult::Help => return Ok(false),
            }
        }
    }

    /// Performs a parse run of a single parser
    ///
    ///  - `options` is the developer provided options to be updated
    ///  - `args` is the argument stream to be parsed from
    ///  - `command_chain` is the list of commands that precede this parser
    ///  - `first_argument` is the first argument passed to the program, usually the path to the program
    ///  - `program_name` is the program name to be used in help, potentially inherited
    ///  - `description` is the program description to be used in help, potentially inherited
    fn do_parse<'a>(
        &'a mut self,
        options: &mut T,
        args: &mut ArgStream,
        command_chain: &[String],
        first_argument: &str,
        program_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<ParseResult<'a, T, E>, Error<E>> {
        // This pointer is stored for a help message. This is required for the borrow checker.
        let self_ptr = self as *const _;

        let (flags, terminal, short_prefix, long_prefix, prologue, epilogue) = self.as_mut();

        while let Some(argument) = args.next_os() {
            let argument = if starts_with(&argument, long_prefix) {
                let arg_name = argument
                    .into_string()
                    .map_err(|string| Error::UnknowArgumentOS(string))?;

                flags
                    .get_long(&arg_name[long_prefix.len()..])
                    .ok_or(Error::UnknowArgument(arg_name))
            } else if starts_with(&argument, short_prefix) {
                let arg_name = argument
                    .into_string()
                    .map_err(|string| Error::UnknowArgumentOS(string))?;

                flags
                    .get_short(&arg_name[short_prefix.len()..])
                    .ok_or(Error::UnknowArgument(arg_name))
            } else {
                if terminal.parse(options, argument)? {
                    break;
                }

                continue;
            }?;

            if let Some(exit) = argument.parse(options, args)? {
                // UNSAFE: The borrow checker won't let us use [`self`] here
                // because of the `self.as_mut()` call above. This immutable
                // borrow is safe.
                print!(
                    "{}",
                    HelpGenerator::new(
                        unsafe { &*(self_ptr as *const _) },
                        command_chain,
                        first_argument,
                        program_name,
                        description,
                        prologue,
                        epilogue
                    )
                );

                if exit {
                    std::process::exit(0);
                } else {
                    return Ok(ParseResult::Help);
                }
            }
        }

        flags.finalize()?;

        terminal.finalize(options).map(|parser| match parser {
            Some((parser, command)) => ParseResult::Continue(parser, command),
            None => ParseResult::Complete,
        })
    }

    fn as_mut<'a>(
        &'a mut self,
    ) -> (
        &'a mut FlagSet<T, E>,
        &'a mut TerminalArgument<T, E>,
        &'a str,
        &'a str,
        Option<&'a str>,
        Option<&'a str>,
    ) {
        (
            &mut self.flag_arguments,
            &mut self.terminal_argument,
            self.short_prefix.as_ref(),
            self.long_prefix.as_ref(),
            self.prologue.as_deref(),
            self.epilogue.as_deref(),
        )
    }
}
