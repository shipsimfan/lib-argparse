mod r#struct;

mod to_tokens;

pub use r#struct::*;

/// The output code from the `FlagGroup` derive macro
pub enum Output<'a> {
    /// Produce the output for a struct
    Struct(StructOutput<'a>),
}
