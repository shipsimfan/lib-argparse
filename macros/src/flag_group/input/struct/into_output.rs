use super::StructInput;
use crate::flag_group::output::{InProgress, NewInProgress, Output, StructOutput};

impl<'a> StructInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let new_in_progress = NewInProgress::new(self.flags.len());

        let mut infos = Vec::with_capacity(self.flags.len());
        let mut types = Vec::with_capacity(self.flags.len());
        let mut long_names = Vec::with_capacity(self.flags.len());
        let mut short_names = Vec::with_capacity(self.flags.len());
        let mut unwraps = Vec::with_capacity(self.flags.len());
        for (i, flag) in self.flags.into_iter().enumerate() {
            let (info, r#type, long_name, short_name, unwrap) = flag.into_output(i);

            infos.push(info);
            r#types.push(r#type);
            long_names.push(long_name);
            unwraps.push(unwrap);

            if let Some(short_name) = short_name {
                short_names.push(short_name);
            }
        }

        let in_progress = InProgress::new(types);

        Output::Struct(StructOutput::new(
            self.name,
            infos,
            in_progress,
            new_in_progress,
            long_names,
            short_names,
            unwraps,
        ))
    }
}
