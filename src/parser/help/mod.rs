use crate::FlagArgument;
use std::ffi::OsString;

mod flags;
mod header;
mod usage;

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

    flags::generate(header, flags, short_prefix, long_prefix);

    if let Some(epilogue) = epilogue {
        println!();
        println!("{}", epilogue);
    }
}
