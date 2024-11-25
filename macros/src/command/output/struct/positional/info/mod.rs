use crate::command::output::r#struct::{DefaultValue, OptionalOutput};
use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};

mod new;
mod to_tokens;

/// The description of a positional
pub struct PositionalInfo<'a> {
    /// The name for the constant
    info_name: Identifier,

    /// The type of the positional
    r#type: Type<'a>,

    /// The name of the value
    value: Literal,

    /// The minimum count
    min_count: Literal,

    /// The maximum count
    max_count: Literal,

    /// The default value
    default: OptionalOutput<DefaultValue<'a>>,
}
