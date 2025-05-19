use crate::positional::{input::Input, output::Output};

impl<'a> Input<'a> {
    /// Converts the details into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        match self {
            Input::Enum(r#enum) => r#enum.into_output(),
        }
    }
}
