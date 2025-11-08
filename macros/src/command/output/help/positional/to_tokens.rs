use super::PositionalHelpOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalHelpOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalHelpOutput {
            info_name,
            description_offset,
        } = self;

        to_tokens! { generator
            Self::#info_name.display_help(#description_offset);
        }
    }
}
