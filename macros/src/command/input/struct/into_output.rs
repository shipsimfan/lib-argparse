use crate::command::{
    input::StructInput,
    output::{Output, StructOutput},
};

impl StructInput {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output {
        Output::new_struct(self.name, StructOutput::new())
    }
}
