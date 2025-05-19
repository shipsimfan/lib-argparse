use super::FlagGroupHelpOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupHelpOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupHelpOutput { header, r#type } = self;

        to_tokens! { generator
            ::std::println!();
            ::std::println!("{}:", #header);
            <#r#type as ::argparse::FlagGroup>::print_help();
        }
    }
}
