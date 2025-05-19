use proc_macro_util::ast::Type;

mod new;
mod to_tokens;

/// Generates the if statement to match a flag group's short names
pub struct FlagGroupShortName<'a> {
    /// The index of the child flag group in the in-progress tuple
    index: usize,

    /// The type of a flag group
    r#type: Type<'a>,
}
