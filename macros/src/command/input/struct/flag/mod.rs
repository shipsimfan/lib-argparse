use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

mod extract;
mod into_output;

/// The information extracted for a flag
pub struct Flag<'a> {
    /// The name of the variable in the struct
    variable_name: Identifier,

    /// The name of the constant describing its info
    info_name: Identifier,

    /// The type of the variable
    r#type: Type<'a>,

    /// The long name for the flag
    long_name: Literal,

    /// The short name for the flag
    short_name: Option<Literal>,

    /// The name of the value for this flag
    value: Option<Literal>,

    /// The minimum count of values
    min_count: Literal,

    /// The maximum count of values
    max_count: Literal,

    /// The default value
    default: Option<Expression<'a>>,
}