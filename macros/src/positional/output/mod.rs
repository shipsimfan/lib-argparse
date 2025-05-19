mod r#enum;

mod to_tokens;

pub use r#enum::*;

/// The output code from the `Positional` derive macro
pub enum Output<'a> {
    /// Produce the output for an enum
    Enum(EnumOutput<'a>),
}
