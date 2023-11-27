use crate::FlagArgument;
use std::ffi::OsString;

mod flags;
mod header;
mod usage;

pub(super) fn generate<Options>(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
    usage: Option<&str>,
    command_list: &[OsString],
    prologue: Option<&dyn std::fmt::Display>,
    header: Option<&dyn std::fmt::Display>,
    flags: &[&dyn FlagArgument<Options>],
    epilogue: Option<&dyn std::fmt::Display>,
) {
    if header::generate(name, description) {
        println!();
    }

    usage::generate(usage, command_list);
    println!();

    if let Some(prologue) = prologue {
        println!("{}", prologue);
        println!();
    }

    // TODO: terminal::generate

    flags::generate(header, flags);

    if let Some(epilogue) = epilogue {
        println!();
        println!("{}", epilogue);
    }
}
