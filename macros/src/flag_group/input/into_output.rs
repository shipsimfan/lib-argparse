use crate::flag_group::{input::Input, output::Output};

impl<'a> Input<'a> {
    /// Converts the details into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        match self {
            Input::Struct(r#struct) => r#struct.into_output(),
        }
    }
}
