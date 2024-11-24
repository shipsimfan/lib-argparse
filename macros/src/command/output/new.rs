use super::{kind::OutputKind, Output, StructOutput};
use proc_macro_util::tokens::Identifier;

impl Output {
    pub fn new_struct(name: Identifier, r#struct: StructOutput) -> Self {
        Output {
            name,
            kind: OutputKind::Struct(r#struct),
        }
    }
}
