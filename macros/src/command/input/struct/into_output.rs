use crate::command::{
    input::StructInput,
    output::{Output, StructOutput},
};

impl<'a> StructInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let mut flag_description_offset = 0;
        let mut short_names = false;
        for flag in &self.flags {
            flag_description_offset = flag_description_offset.max(flag.help_length());
            short_names |= flag.short_name().is_some();
        }

        if short_names {
            flag_description_offset += 3;
        }

        flag_description_offset += 2;

        let mut flag_info = Vec::with_capacity(self.flags.len());
        let mut flag_declarations = Vec::with_capacity(self.flags.len());
        let mut flag_long_names = Vec::with_capacity(self.flags.len());
        let mut flag_short_names = Vec::with_capacity(self.flags.len());
        let mut flag_unwraps = Vec::with_capacity(self.flags.len());
        let mut flag_usages = Vec::with_capacity(self.flags.len());
        let mut flag_help = Vec::with_capacity(self.flags.len());
        for flag in self.flags {
            let (info, declaration, long_name, short_name, unwrap, usage, help) =
                flag.into_output(flag_description_offset, short_names);
            flag_info.push(info);
            flag_declarations.push(declaration);
            flag_long_names.push(long_name);
            flag_unwraps.push(unwrap);
            flag_usages.push(usage);
            flag_help.push(help);

            if let Some(short_name) = short_name {
                flag_short_names.push(short_name);
            }
        }

        let mut positional_description_offset = 0;
        for positional in &self.positionals {
            positional_description_offset =
                positional_description_offset.max(positional.help_length());
        }

        positional_description_offset += 2;

        let mut positional_info = Vec::with_capacity(self.positionals.len());
        let mut positional_declarations = Vec::with_capacity(self.positionals.len());
        let mut positional_matches = Vec::with_capacity(self.positionals.len());
        let mut positional_sub_commands = Vec::with_capacity(self.positionals.len());
        let mut positional_unwraps = Vec::with_capacity(self.positionals.len());
        let mut positional_usages = Vec::with_capacity(self.positionals.len());
        let mut positional_help = Vec::with_capacity(self.positionals.len());
        for (index, positional) in self.positionals.into_iter().enumerate() {
            let (info, declaration, r#match, sub_command, unwrap, usage, help) =
                positional.into_output(index, positional_description_offset);
            positional_info.push(info);
            positional_declarations.push(declaration);
            positional_matches.push(r#match);
            positional_sub_commands.push(sub_command);
            positional_unwraps.push(unwrap);
            positional_usages.push(usage);
            positional_help.push(help);
        }

        let mut flag_group_declarations = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_long_names = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_short_names = Vec::with_capacity(self.flag_groups.len());
        let mut flag_group_unwraps = Vec::with_capacity(self.flag_groups.len());
        for flag_group in self.flag_groups {
            let (declaration, long_name, short_name, unwrap) = flag_group.into_output();

            flag_group_declarations.push(declaration);
            flag_group_long_names.push(long_name);
            flag_group_short_names.push(short_name);
            flag_group_unwraps.push(unwrap);
        }

        let (version, help) =
            self.info
                .into_output(positional_usages, positional_help, flag_usages, flag_help);

        Output::new_struct(StructOutput::new(
            self.name,
            positional_info,
            positional_declarations,
            positional_matches,
            positional_sub_commands,
            positional_unwraps,
            flag_info,
            flag_declarations,
            flag_long_names,
            flag_short_names,
            flag_unwraps,
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            version,
            help,
        ))
    }
}
