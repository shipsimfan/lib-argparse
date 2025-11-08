use super::FlagUnwrap;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagUnwrap<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagUnwrap {
            variable_name,
            info_name,
        } = self;
        let variable_name2 = variable_name.clone();

        to_tokens! { generator
            #variable_name: ::argparse::Flag::unwrap(#variable_name2, Self::#info_name)?,
        }
    }
}
