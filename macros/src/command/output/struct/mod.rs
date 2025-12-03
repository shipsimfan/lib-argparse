use super::{HelpOutput, VersionOutput};
use proc_macro_util::{
    ast::{GenericArgs, GenericParams},
    tokens::Identifier,
};
use std::borrow::Cow;

mod default_value;
mod description;
mod flag;
mod flag_group;
mod optional_output;
mod positional;
mod variable_declaration;

mod new;
mod to_tokens;

pub use default_value::DefaultValue;
pub use description::Description;
pub use flag::{FlagInfo, FlagLongName, FlagShortName, FlagUnwrap};
pub use flag_group::{
    FlagGroupDeclaration, FlagGroupLongName, FlagGroupShortName, FlagGroupUnwrap,
};
pub use optional_output::OptionalOutput;
pub use positional::{PositionalInfo, PositionalMatch, PositionalSubCommand, PositionalUnwrap};
pub use variable_declaration::VariableDeclaration;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The generic params attached to the struct
    generic_params: Option<GenericParams<'a>>,

    /// The generic arguments attached to the struct
    generic_args: Option<GenericArgs<'a>>,

    /// The name of the module to produce the implementation in
    module_name: Identifier,

    /// The info describing the positionals
    positional_info: Vec<PositionalInfo<'a>>,

    /// Declaration of positional variables
    positional_declarations: Vec<VariableDeclaration<'a>>,

    /// The match arms for positionals
    positional_matches: Vec<PositionalMatch<'a>>,

    /// The match arms for positional sub commands
    positional_sub_commands: Vec<PositionalSubCommand<'a>>,

    /// Unwrapping of positional variables
    positional_unwraps: Vec<PositionalUnwrap<'a>>,

    /// The info describing the flags
    flag_info: Vec<FlagInfo<'a>>,

    /// Declaration of flag variables
    flag_declarations: Vec<VariableDeclaration<'a>>,

    /// Match arms for flag long names
    flag_long_names: Vec<FlagLongName<'a>>,

    /// Match arms for flag short names
    flag_short_names: Vec<FlagShortName<'a>>,

    /// Unwrapping of flag variables
    flag_unwraps: Vec<FlagUnwrap<'a>>,

    /// Declaration of flag group variables
    flag_group_declarations: Vec<FlagGroupDeclaration<'a>>,

    /// The if statements for matching flag groups based on a long name
    flag_group_long_names: Vec<FlagGroupLongName<'a>>,

    /// The if statements for matching flag groups based on a short name
    flag_group_short_names: Vec<FlagGroupShortName<'a>>,

    /// Unwraps the flag group variables
    flag_group_unwraps: Vec<FlagGroupUnwrap<'a>>,

    /// The version flag to output
    version: Option<VersionOutput<'a>>,

    /// The help flag to output
    help: Option<HelpOutput<'a>>,
}
