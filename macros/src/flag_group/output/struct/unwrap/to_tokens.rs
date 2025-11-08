use super::FlagUnwrap;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagUnwrap<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagUnwrap {
            variable_name,
            index,
            info_name,
        } = self;

        to_tokens! { generator
            #variable_name: ::argparse::Flag::unwrap(this.#index, Self::#info_name)?,
        }
    }
}
