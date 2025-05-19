use super::FlagGroupShortName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupShortName<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupShortName {
            variable_name,
            r#type,
        } = self;

        to_tokens! { generator
            if <#r#type as ::argparse::FlagGroup>::parse_short(&mut #variable_name, __flag_name, __source)? {
                continue;
            }
        }
    }
}
