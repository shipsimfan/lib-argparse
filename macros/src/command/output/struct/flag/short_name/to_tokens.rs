use super::FlagShortName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagShortName<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagShortName {
            short_name,
            variable_name,
            info_name,
        } = self;

        to_tokens! { generator
            #short_name => ::argparse::Flag::parse(&mut #variable_name, __source, #info_name, false)?,
        }
    }
}
