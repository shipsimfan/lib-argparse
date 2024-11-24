use proc_macro_util::tokens::Identifier;

mod default_value;
mod positional;

mod new;
mod to_tokens;

pub use default_value::DefaultValue;
pub use positional::PositionalInfo;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    pub(super) name: Identifier,

    /// The info describing the positionals
    positional_info: Vec<PositionalInfo<'a>>,
}
