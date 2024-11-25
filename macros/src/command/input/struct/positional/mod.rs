use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

mod extract;
mod into_output;

/// The information extracted for a positional
pub struct Positional<'a> {
    /// The name of the variable in the struct
    variable_name: Identifier,

    /// The name of the constant describing its info
    info_name: Identifier,

    /// The type of the variable
    r#type: Type<'a>,

    /// The name for the value of the positional
    value: Literal,

    /// The minimum count
    min_count: Literal,

    /// The maximum count
    max_count: Literal,

    /// The default value
    default: Option<Expression<'a>>,
}
