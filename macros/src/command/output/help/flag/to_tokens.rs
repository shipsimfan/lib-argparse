use super::FlagHelpOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagHelpOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagHelpOutput {
            info_name,
            description_offset,
            short_names,
        } = self;

        to_tokens! { generator
            #info_name.display_help(#short_names, #description_offset);
        }
    }
}
