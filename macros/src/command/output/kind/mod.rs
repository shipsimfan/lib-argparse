use super::StructOutput;

mod to_tokens;

/// The kind of output to produce
pub enum OutputKind<'a> {
    /// Produce the output for a struct
    Struct(StructOutput<'a>),
}
