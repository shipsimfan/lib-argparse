use proc_macro_util::tokens::Identifier;

mod default_value;
mod flag;
mod optional_output;
mod positional;
mod variable_declaration;

mod new;
mod to_tokens;

pub use default_value::DefaultValue;
pub use flag::{FlagInfo, FlagUnwrap};
pub use optional_output::OptionalOutput;
pub use positional::{PositionalInfo, PositionalUnwrap};
pub use variable_declaration::VariableDeclaration;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    pub(super) name: Identifier,

    /// The info describing the positionals
    positional_info: Vec<PositionalInfo<'a>>,

    /// Declaration of positional variables
    positional_declarations: Vec<VariableDeclaration>,

    /// Unwrapping of positional variables
    positional_unwraps: Vec<PositionalUnwrap>,

    /// The info describing the flags
    flag_info: Vec<FlagInfo<'a>>,

    /// Declaration of flag variables
    flag_declarations: Vec<VariableDeclaration>,

    /// Unwrapping of flag variables
    flag_unwraps: Vec<FlagUnwrap>,
}
