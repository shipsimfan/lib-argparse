use super::FlagLongName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for FlagLongName {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagLongName {
            long_name,
            variable_name,
            info_name,
        } = self;

        to_tokens! { generator
            #long_name => ::argparse::Flag::parse(&mut #variable_name, __source, #info_name, true)?,
        }
    }
}
