use proc_macro_util::tokens::Identifier;

mod default_value;
mod flag;
mod optional_output;
mod positional;

mod new;
mod to_tokens;

pub use default_value::DefaultValue;
pub use flag::FlagInfo;
pub use optional_output::OptionalOutput;
pub use positional::PositionalInfo;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    pub(super) name: Identifier,

    /// The info describing the positionals
    positional_info: Vec<PositionalInfo<'a>>,

    /// The info describing the flags
    flag_info: Vec<FlagInfo<'a>>,
}
