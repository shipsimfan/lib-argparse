use crate::command::output::{AsF64, DefaultValue, Description, OptionalOutput};
use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

mod new;
mod to_tokens;

/// The description of a flag
pub struct FlagInfo<'a> {
    /// The name for the constant
    info_name: Identifier,

    /// The type of the flag
    r#type: Type<'a>,

    /// The long name for the flag
    info_long_name: OptionalOutput<Literal>,

    /// The short name for the flag
    info_short_name: OptionalOutput<Literal>,

    /// The name of the value
    value: OptionalOutput<Literal>,

    /// The minimum length/quantity/value
    min: OptionalOutput<AsF64<Expression<'a>>>,

    /// The maximum length/quantity/value
    max: OptionalOutput<AsF64<Expression<'a>>>,

    /// The default value
    default: OptionalOutput<DefaultValue<'a>>,

    /// The description
    description: OptionalOutput<Description<'a>>,
}
