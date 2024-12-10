use super::PositionalHelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalHelpUsageOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalHelpUsageOutput { info_name } = self;

        to_tokens! { generator
            #info_name.display_usage();
        }
    }
}
