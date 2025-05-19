use super::PositionalUnwrap;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for PositionalUnwrap<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalUnwrap {
            variable_name,
            info_name,
        } = self;
        let variable_name2 = variable_name.clone();

        to_tokens! { generator
            #variable_name: ::argparse::Positional::unwrap(#variable_name2, #info_name)?,
        }
    }
}
