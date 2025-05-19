use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};
use std::borrow::Cow;

mod extract;
mod into_output;

/// The information extracted for a flag group
pub struct FlagGroup<'a> {
    /// The name of the variable in the struct
    variable_name: Cow<'a, Identifier>,

    /// The type of the group
    r#type: Type<'a>,

    /// The header for the group in help
    header_name: Literal,
}
