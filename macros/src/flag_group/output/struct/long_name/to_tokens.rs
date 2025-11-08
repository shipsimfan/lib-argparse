use super::FlagLongName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagLongName {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagLongName {
            long_name,
            index,
            info_name,
        } = self;

        to_tokens! { generator
            #long_name => ::argparse::Flag::parse(&mut this.#index, source, Self::#info_name, true)?,
        }
    }
}
