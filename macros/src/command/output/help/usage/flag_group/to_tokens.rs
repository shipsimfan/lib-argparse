use super::FlagGroupHelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupHelpUsageOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupHelpUsageOutput { r#type } = self;

        to_tokens! { generator
            __optional_flags |= <#r#type as ::argparse::FlagGroup>::print_help_usage();
        }
    }
}
