use super::StructInput;
use crate::flag_group::output::{InProgress, NewInProgress, Output, StructOutput};

impl<'a> StructInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let new_in_progress = NewInProgress::new(self.flags.len());

        let mut description_offset = 0;
        let mut has_short_names = false;
        for flag in &self.flags {
            description_offset = description_offset.max(flag.help_length());
            has_short_names |= flag.short_name().is_some();
        }

        if has_short_names {
            description_offset += 3;
        }

        description_offset += 2;

        let num_flags = self.flags.len();

        let mut infos = Vec::with_capacity(self.flags.len());
        let mut types = Vec::with_capacity(self.flags.len());
        let mut long_names = Vec::with_capacity(self.flags.len());
        let mut short_names = Vec::with_capacity(self.flags.len());
        let mut unwraps = Vec::with_capacity(self.flags.len());
        let mut usages = Vec::with_capacity(self.flags.len());
        let mut helps = Vec::with_capacity(self.flags.len());
        for (i, flag) in self.flags.into_iter().enumerate() {
            let (info, r#type, long_name, short_name, unwrap, usage, help) =
                flag.into_output(i, description_offset, has_short_names);

            infos.push(info);
            r#types.push(r#type);
            long_names.push(long_name);
            unwraps.push(unwrap);
            usages.push(usage);
            helps.push(help);

            if let Some(short_name) = short_name {
                short_names.push(short_name);
            }
        }

        let mut flag_group_in_progress = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_declarations = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_long_names = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_short_names = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_unwraps = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_usages = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_helps = Vec::with_capacity(self.flag_groups.len());
        for (i, flag_group) in self.flag_groups.into_iter().enumerate() {
            let (in_progress, declaration, long_name, short_name, unwrap, usage, help) =
                flag_group.into_output(i + num_flags);

            flag_group_in_progress.push(in_progress);
            flag_group_declarations.push(declaration);
            flag_group_long_names.push(long_name);
            flag_group_short_names.push(short_name);
            flag_group_unwraps.push(unwrap);
            flag_group_usages.push(usage);
            flag_group_helps.push(help);
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
            usages,
            helps,
            flag_group_in_progress,
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            flag_group_usages,
            flag_group_helps,
        ))
    }
}
