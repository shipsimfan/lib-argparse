use crate::{ArgStream, Command, Error, FlagArgument, FlagSet, Positionals, TerminalArgument};
use std::{borrow::Cow, ffi::OsStr, ops::Deref};

/// Argument Parser
pub struct Parser<T, E: 'static = ()> {
    /// Program name to display for help
    program_name: Option<Cow<'static, str>>,
    /// Program description for help
    description: Option<Cow<'static, str>>,

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
    /// Creates a new empty `Parser`
    pub fn new() -> Self {
        Parser {
            program_name: None,
            description: None,

            short_prefix: "-".into(),
            long_prefix: "--".into(),

            flag_arguments: FlagSet::new(),
            terminal_argument: TerminalArgument::None,
        }
    }

    /// Returns the program name
    pub fn program_name(&self) -> Option<&str> {
        self.program_name
            .as_ref()
            .map(|long_name| long_name.deref())
    }

    /// Returns the program description
    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|long_name| long_name.deref())
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
    ///  - `program_name` is the string the program name will be set to
    pub fn set_program_name<S: Into<Cow<'static, str>>>(&mut self, program_name: S) {
        self.program_name = Some(program_name.into());
    }

    /// Sets the program description
    ///
    ///  - `description` is the string the program description will be set to
    pub fn set_description<S: Into<Cow<'static, str>>>(&mut self, description: S) {
        self.description = Some(description.into());
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
    pub fn set_terminal_argument(&mut self, terminal_argument: TerminalArgument<T, E>) {
        self.terminal_argument = terminal_argument;
    }

    /// Sets the terminal argument to a `Command`
    ///
    ///  - `command` is the `Command` the terminal argument will be set to
    pub fn set_terminal_command(&mut self, command: Command<T, E>) {
        self.terminal_argument = TerminalArgument::Command(command);
    }

    /// Sets the terminal arugment to a `Positionals`
    ///
    ///  - `positionals` is the `Positionals` the terminal argument will be set to
    pub fn set_terminal_positionals(&mut self, positionals: Positionals<T, E>) {
        self.terminal_argument = TerminalArgument::Positionals(positionals);
    }

    /// Removes the terminal argument
    pub fn clear_terminal_argument(&mut self) {
        self.terminal_argument = TerminalArgument::None;
    }

    /// Sets the prefix for short names
    ///
    ///  - `short_prefix` is the string the short prefix will be set to
    pub fn set_short_prefix<S: Into<Cow<'static, str>>>(&mut self, short_prefix: S) {
        let short_prefix = short_prefix.into();
        assert_ne!(
            self.long_prefix, short_prefix,
            "Short prefix and long prefix cannot be the same"
        );
        self.short_prefix = short_prefix;
    }

    /// Sets the prefix for long names
    ///
    ///  - `long_prefix` is the string the long prefix will be set to
    pub fn set_long_prefix<S: Into<Cow<'static, str>>>(&mut self, long_prefix: S) {
        let long_prefix = long_prefix.into();
        assert_ne!(
            self.short_prefix, long_prefix,
            "Short prefix and long prefix cannot be the same"
        );
        self.long_prefix = long_prefix;
    }

    /// Parses arguments from the environment
    ///
    ///  - `options` is the developer provided options to be updated
    pub fn parse(&mut self, mut options: T) -> Result<T, Error<E>> {
        let mut args = ArgStream::new();

        let mut parser = self;
        while let Some(new_parser) = parser.do_parse(&mut options, &mut args)? {
            parser = new_parser;
        }

        Ok(options)
    }

    /// Performs a parse run of a single parser
    ///
    ///  - `options` is the developer provided options to be updated
    ///  - `args` is the argument stream to be parsed from
    fn do_parse<'a>(
        &'a mut self,
        options: &mut T,
        args: &mut ArgStream,
    ) -> Result<Option<&'a mut Parser<T, E>>, Error<E>> {
        let (flags, terminal, short_prefix, long_prefix) = self.as_mut();

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

            if argument.parse(options, args)? {
                todo!("Generate help")
            }
        }

        flags.finalize()?;
        terminal.finalize(options)
    }

    fn as_mut<'a>(
        &'a mut self,
    ) -> (
        &'a mut FlagSet<T, E>,
        &'a mut TerminalArgument<T, E>,
        &str,
        &str,
    ) {
        (
            &mut self.flag_arguments,
            &mut self.terminal_argument,
            self.short_prefix.as_ref(),
            self.long_prefix.as_ref(),
        )
    }
}
