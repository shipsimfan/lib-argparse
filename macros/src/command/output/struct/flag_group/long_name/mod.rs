use proc_macro_util::{ast::Type, tokens::Identifier};
use std::borrow::Cow;

mod new;
mod to_tokens;

/// Generates the if statement to match a flag group's long names
pub struct FlagGroupLongName<'a> {
    /// The name of this flag's variable
    variable_name: Cow<'a, Identifier>,

    /// The type of a flag group
    r#type: Type<'a>,
}
