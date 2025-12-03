use super::{Output, StructOutput};

impl<'a> Output<'a> {
    pub fn new_struct(r#struct: StructOutput<'a>) -> Self {
        Output::Struct(r#struct)
    }
}
