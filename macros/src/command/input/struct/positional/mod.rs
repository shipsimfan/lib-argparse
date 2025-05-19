use std::borrow::Cow;

use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

mod extract;
mod help_length;
mod into_output;

/// The information extracted for a positional
pub struct Positional<'a> {
    /// The name of the variable in the struct
    variable_name: Cow<'a, Identifier>,

    /// The name of the constant describing its info
    info_name: Identifier,

    /// The type of the variable
    r#type: Type<'a>,

    /// The name for the value of the positional
    value: Literal,

    /// The minimum length/quantity/value
    min: Option<Expression<'a>>,

    /// The maximum length/quantity/value
    max: Option<Expression<'a>>,

    /// The default value
    default: Option<Expression<'a>>,

    /// The description of this positional
    description: Option<Vec<Expression<'static>>>,
}
