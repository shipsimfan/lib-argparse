use super::PositionalMatch;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalMatch {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalMatch {
            index,
            variable_name,
            info_name,
        } = self;

        to_tokens! { generator
            #index => ::argparse::Positional::parse(&mut #variable_name, __argument.clone(), #info_name),
        }
    }
}
