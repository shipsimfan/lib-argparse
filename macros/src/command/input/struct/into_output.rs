use super::{Flag, Positional};
use crate::command::{
    input::StructInput,
    output::{Output, StructOutput},
};

impl<'a> StructInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let positional_info = self
            .positionals
            .into_iter()
            .map(Positional::into_output)
            .collect();

        let flag_info = self.flags.into_iter().map(Flag::into_output).collect();

        Output::new_struct(StructOutput::new(self.name, positional_info, flag_info))
    }
}
