use crate::command::{
    input::StructInput,
    output::{Output, StructOutput},
};

impl<'a> StructInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let mut positional_info = Vec::with_capacity(self.positionals.len());
        let mut positional_declarations = Vec::with_capacity(self.positionals.len());
        let mut positional_matches = Vec::with_capacity(self.positionals.len());
        let mut positional_sub_commands = Vec::with_capacity(self.positionals.len());
        let mut positional_unwraps = Vec::with_capacity(self.positionals.len());
        for (index, positional) in self.positionals.into_iter().enumerate() {
            let (info, declaration, r#match, sub_command, unwrap) = positional.into_output(index);
            positional_info.push(info);
            positional_declarations.push(declaration);
            positional_matches.push(r#match);
            positional_sub_commands.push(sub_command);
            positional_unwraps.push(unwrap);
        }

        let mut flag_info = Vec::with_capacity(self.flags.len());
        let mut flag_declarations = Vec::with_capacity(self.flags.len());
        let mut flag_long_names = Vec::with_capacity(self.flags.len());
        let mut flag_short_names = Vec::with_capacity(self.flags.len());
        let mut flag_unwraps = Vec::with_capacity(self.flags.len());
        for flag in self.flags {
            let (info, declaration, long_name, short_name, unwrap) = flag.into_output();
            flag_info.push(info);
            flag_declarations.push(declaration);
            flag_long_names.push(long_name);
            flag_unwraps.push(unwrap);

            if let Some(short_name) = short_name {
                flag_short_names.push(short_name);
            }
        }

        let version = self.info.into_output();

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
            version,
        ))
    }
}
