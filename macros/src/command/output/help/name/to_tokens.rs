use super::HelpOutputName;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for HelpOutputName {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            HelpOutputName::Default => {
                to_tokens! { generator
                    ::std::env!("CARGO_PKG_NAME")
                }
            }
            HelpOutputName::Provided(literal) => literal.to_tokens(generator),
        }
    }
}
