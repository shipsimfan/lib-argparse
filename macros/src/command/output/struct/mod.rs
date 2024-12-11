use super::{HelpOutput, VersionOutput};
use proc_macro_util::tokens::Identifier;

mod default_value;
mod description;
mod flag;
mod optional_output;
mod positional;
mod variable_declaration;

mod new;
mod to_tokens;

pub use default_value::DefaultValue;
pub use description::Description;
pub use flag::{FlagInfo, FlagLongName, FlagShortName, FlagUnwrap};
pub use optional_output::OptionalOutput;
pub use positional::{PositionalInfo, PositionalMatch, PositionalSubCommand, PositionalUnwrap};
pub use variable_declaration::VariableDeclaration;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    pub(super) name: Identifier,

    /// The info describing the positionals
    positional_info: Vec<PositionalInfo<'a>>,

    /// Declaration of positional variables
    positional_declarations: Vec<VariableDeclaration>,

    /// The match arms for positionals
    positional_matches: Vec<PositionalMatch>,

    /// The match arms for positional sub commands
    positional_sub_commands: Vec<PositionalSubCommand>,

    /// Unwrapping of positional variables
    positional_unwraps: Vec<PositionalUnwrap>,

    /// The info describing the flags
    flag_info: Vec<FlagInfo<'a>>,

    /// Declaration of flag variables
    flag_declarations: Vec<VariableDeclaration>,

    /// Match arms for flag long names
    flag_long_names: Vec<FlagLongName>,

    /// Match arms for flag short names
    flag_short_names: Vec<FlagShortName>,

    /// Unwrapping of flag variables
    flag_unwraps: Vec<FlagUnwrap>,

    /// The version flag to output
    version: Option<VersionOutput<'a>>,

    /// The help flag to output
    help: Option<HelpOutput<'a>>,
}
