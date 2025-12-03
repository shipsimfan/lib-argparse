use super::CommandInfo;
use flag::Flag;
use flag_group::FlagGroup;
use positional::Positional;
use proc_macro_util::{ast::GenericParams, tokens::Identifier};
use std::borrow::Cow;

mod flag;
mod flag_group;
mod positional;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The generic parameters attached to the struct
    generic_params: Option<GenericParams<'a>>,

    /// The positionals in this struct
    positionals: Vec<Positional<'a>>,

    /// The flags in this struct
    flags: Vec<Flag<'a>>,

    /// The flag groups in this struct
    flag_groups: Vec<FlagGroup<'a>>,

    /// The information describing the command
    info: CommandInfo<'a>,
}
