use r#enum::EnumInput;

mod r#enum;

mod extract;
mod into_output;

/// The details extracted from the input item to the Positional derive macro
pub enum Input<'a> {
    /// The input item is an enum
    Enum(EnumInput<'a>),
}
