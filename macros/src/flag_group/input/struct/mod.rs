use flag::Flag;
use flag_group::FlagGroup;
use proc_macro_util::{ast::GenericParams, tokens::Identifier};
use std::borrow::Cow;

mod flag;
mod flag_group;

mod extract;
mod into_output;

/// The details extracted from a struct
pub struct StructInput<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The generic parameters attached to the struct
    generic_params: Option<GenericParams<'a>>,

    /// The flags in the struct
    flags: Vec<Flag<'a>>,

    /// Child flag groups of this flag group
    flag_groups: Vec<FlagGroup<'a>>,
}
