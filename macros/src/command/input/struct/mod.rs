use super::CommandInfo;
use flag::Flag;
use positional::Positional;
use proc_macro_util::tokens::Identifier;

mod flag;
mod positional;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput<'a> {
    /// The name of the struct
    name: Identifier,

    /// The positionals in this struct
    positionals: Vec<Positional<'a>>,

    /// The flags in this struct
    flags: Vec<Flag<'a>>,

    /// The information describing the command
    info: CommandInfo<'a>,
}
