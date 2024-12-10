use super::FlagHelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagHelpUsageOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagHelpUsageOutput { info_name } = self;

        to_tokens! { generator
            __optional_flags |= #info_name.display_usage();
        }
    }
}
