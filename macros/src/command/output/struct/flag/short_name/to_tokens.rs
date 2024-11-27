use super::FlagShortName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagShortName {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagShortName {
            short_name,
            variable_name,
            info_name,
        } = self;

        to_tokens! { generator
            #short_name => #variable_name = Some(::argparse::Flag::parse(__source, #info_name, false)?),
        }
    }
}
