use super::{kind::OutputKind, Output, StructOutput};

impl Output {
    pub fn new_struct(r#struct: StructOutput) -> Self {
        Output {
            name: r#struct.name.clone(),
            kind: OutputKind::Struct(r#struct),
        }
    }
}
