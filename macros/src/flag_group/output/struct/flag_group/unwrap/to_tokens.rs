use super::FlagGroupUnwrap;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupUnwrap<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupUnwrap {
            variable_name,
            index,
        } = self;

        to_tokens! { generator
            #variable_name: ::argparse::FlagGroup::unwrap(this.#index)?,
        }
    }
}
