use super::FlagShortName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagShortName {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagShortName {
            short_name,
            index,
            info_name,
        } = self;

        to_tokens! { generator
            #short_name => ::argparse::Flag::parse(&mut this.#index, source, #info_name, false)?,
        }
    }
}
