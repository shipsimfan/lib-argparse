use kind::OutputKind;
use proc_macro_util::tokens::Identifier;

mod kind;
mod r#struct;

mod new;
mod to_tokens;

pub use r#struct::{DefaultValue, PositionalInfo, StructOutput};

/// The output code from the Command derive macro
pub struct Output<'a> {
    /// The name of the input item
    name: Identifier,

    /// The kind of output to produce
    kind: OutputKind<'a>,
}
