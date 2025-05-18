use crate::command::output::{
    as_f64::AsF64,
    r#struct::{DefaultValue, Description, OptionalOutput},
};
use proc_macro_util::{
    ast::{Expression, Type},
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

    /// The minimum length/quantity/value
    min: OptionalOutput<AsF64<Expression<'a>>>,

    /// The maximum length/quantity/value
    max: OptionalOutput<AsF64<Expression<'a>>>,

    /// The default value
    default: OptionalOutput<DefaultValue<'a>>,

    /// The description of this positional
    description: OptionalOutput<Description<'a>>,
}
