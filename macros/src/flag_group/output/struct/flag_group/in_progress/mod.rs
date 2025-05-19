use proc_macro_util::ast::Type;

mod new;
mod to_tokens;

/// A declaration of a type for a flag group to be used in the in-progress
pub struct FlagGroupInProgress<'a> {
    /// Is this the first element being generated?
    first: bool,

    /// The type of the flag group
    r#type: Type<'a>,
}
