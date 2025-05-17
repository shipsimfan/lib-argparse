use super::HelpOutputDescription;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpOutputDescription<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            HelpOutputDescription::Default => {
                to_tokens! { generator
                   |_| { println!("{}", ::std::env!("CARGO_PKG_DESCRIPTION")); }
                }
            }
            HelpOutputDescription::Provided(expression) => expression.to_tokens(generator),
        }
    }
}
