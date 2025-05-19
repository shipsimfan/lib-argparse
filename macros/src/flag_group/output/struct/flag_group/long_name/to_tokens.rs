use super::FlagGroupLongName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupLongName<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupLongName { index, r#type } = self;

        to_tokens! { generator
            if <#r#type as ::argparse::FlagGroup>::parse_long(&mut this.#index, flag, source)? {
                return Ok(true);
            }
        }
    }
}
