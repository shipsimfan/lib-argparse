use crate::FlagArgument;
use std::{
    ffi::OsString,
    io::{StdoutLock, Write},
};

mod flags;
mod header;
mod usage;

struct StdOut<'a>(StdoutLock<'a>);

pub(super) fn generate<'a, Options>(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
    usage: Option<&str>,
    command_list: &[OsString],
    prologue: Option<&dyn std::fmt::Display>,
    header: Option<&dyn std::fmt::Display>,
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
    epilogue: Option<&dyn std::fmt::Display>,
    short_prefix: &str,
    long_prefix: &str,
) {
    if header::generate(name, description) {
        println!();
    }

    usage::generate(usage, command_list);

    if let Some(prologue) = prologue {
        println!();
        println!("{}", prologue);
    }

    // TODO: terminal::generate

    flags::generate(header, flags, short_prefix, long_prefix, &mut StdOut::new()).unwrap();

    if let Some(epilogue) = epilogue {
        println!();
        println!("{}", epilogue);
    }
}

impl<'a> StdOut<'a> {
    pub(self) fn new() -> Self {
        StdOut(std::io::stdout().lock())
    }
}

impl<'a> std::fmt::Write for StdOut<'a> {
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.0.write_fmt(args).map_err(|_| std::fmt::Error)
    }

    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write_all(s.as_bytes()).map_err(|_| std::fmt::Error)
    }
}
