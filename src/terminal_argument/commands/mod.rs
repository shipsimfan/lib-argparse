use crate::{Error, Parser, Result, TerminalArgument};

mod command;

pub use command::Command;

/// A list of commands
pub struct Commands<'a, Options: 'a> {
    name: Option<&'a dyn std::fmt::Display>,
    required: Option<&'a dyn std::fmt::Display>,
    commands: &'a [Command<'a, Options>],
}

impl<'a, Options: 'a> Commands<'a, Options> {
    /// Creates a new empty list of commands
    pub const fn new(commands: &'a [Command<'a, Options>]) -> Self {
        Commands {
            name: None,
            required: None,
            commands,
        }
    }

    /// Sets the name, which used as the hint and the header for the commands in the help
    pub const fn name(mut self, name: &'a dyn std::fmt::Display) -> Self {
        self.name = Some(name);
        self
    }

    /// Sets this command to be required
    pub const fn required(mut self, missing: &'a dyn std::fmt::Display) -> Self {
        self.required = Some(missing);
        self
    }

    /// Sets this command to be not required
    pub const fn not_required(mut self) -> Self {
        self.required = None;
        self
    }

    fn get_name(&self) -> &dyn std::fmt::Display {
        self.name.unwrap_or(&"COMMAND")
    }
}

impl<'a, Options: 'a> TerminalArgument<'a, Options> for Commands<'a, Options> {
    fn action(
        &self,
        options: &mut Options,
        _: usize,
        parameter: std::ffi::OsString,
    ) -> Result<'a, Option<&Parser<'a, Options>>> {
        let parameter = parameter.into_string()?;

        for command in self.commands {
            if command.get_name() == parameter {
                return command.do_action(options);
            }
        }

        Err(Error::unexpected_argument(format!(
            "unknown command \"{}\"",
            parameter
        )))
    }

    fn finalize(&self, count: usize) -> Result<'a, ()> {
        if let Some(missing) = self.required {
            if count == 0 {
                return Err(Error::missing_required(missing.to_string()));
            }
        }

        Ok(())
    }

    fn help(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if self.commands.len() == 0 {
            return Ok(());
        }

        let mut longest_command = 0;
        for command in self.commands {
            if command.get_name().len() > longest_command {
                longest_command = command.get_name().len();
            }
        }

        writeln!(f, "{}:", self.get_name())?;
        for command in self.commands {
            write!(f, "  {}", command.get_name())?;
            for _ in command.get_name().len()..longest_command {
                write!(f, " ")?;
            }

            let description = match command.get_description() {
                None => {
                    writeln!(f)?;
                    continue;
                }
                Some(description) => description,
            };

            if description.len() == 0 {
                writeln!(f)?;
                continue;
            }

            writeln!(f, "  {}", description[0])?;
            for i in 1..description.len() {
                for _ in 0..longest_command + 4 {
                    write!(f, " ")?;
                }

                writeln!(f, "{}", description[i])?;
            }
        }

        Ok(())
    }

    fn hint(&self) -> &dyn std::fmt::Display {
        self.get_name()
    }
}
