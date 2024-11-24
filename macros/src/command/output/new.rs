use super::{kind::OutputKind, Output, StructOutput};

impl<'a> Output<'a> {
    pub fn new_struct(r#struct: StructOutput<'a>) -> Self {
        Output {
            name: r#struct.name.clone(),
            kind: OutputKind::Struct(r#struct),
        }
    }
}
