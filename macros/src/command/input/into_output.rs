use crate::command::{input::Input, output::Output};

impl Input {
    /// Converts the details into an [`Output`]
    pub fn into_output(self) -> Output {
        match self {
            Input::Struct(r#struct) => r#struct.into_output(),
        }
    }
}
