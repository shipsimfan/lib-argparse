use crate::{terminal_argument::TerminalArgument, FlagArgument};
use std::{ffi::OsString, io::StdoutLock};

mod flags;
mod header;
mod usage;

pub(super) struct StdOut<'a>(StdoutLock<'a>);

pub(super) fn generate<'a, Options>(
    name: Option<&dyn std::fmt::Display>,
    description: &[&dyn std::fmt::Display],
    usage: Option<&str>,
    command_list: &[OsString],
    prologue: Option<&dyn std::fmt::Display>,
    header: Option<&dyn std::fmt::Display>,
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
    epilogue: Option<&dyn std::fmt::Display>,
    short_prefix: &str,
    long_prefix: &str,
    terminal: Option<&dyn TerminalArgument<'a, Options>>,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    if header::generate(name, description, output)? {
        writeln!(output)?;
    }

    usage::generate(
        usage,
        terminal.map(|terminal| terminal.hint()),
        command_list,
        output,
    )?;

    if let Some(prologue) = prologue {
        writeln!(output)?;
        writeln!(output, "{}", prologue)?;
    }

    if let Some(terminal) = terminal {
        writeln!(output)?;
        terminal.help(output)?;
    }

    flags::generate(header, flags, short_prefix, long_prefix, output)?;

    if let Some(epilogue) = epilogue {
        writeln!(output)?;
        writeln!(output, "{}", epilogue)?;
    }

    Ok(())
}

impl<'a> StdOut<'a> {
    pub(super) fn new() -> Self {
        StdOut(std::io::stdout().lock())
    }
}

impl<'a> std::fmt::Write for StdOut<'a> {
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        use std::io::Write;
        self.0.write_fmt(args).map_err(|_| std::fmt::Error)
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        use std::io::Write;
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}
