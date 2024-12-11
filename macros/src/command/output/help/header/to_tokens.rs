use super::HelpHeader;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpHeader<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            ::std::println!();
        }

        match self {
            HelpHeader::UserProvided(expression) => {
                to_tokens! {    generator
                    ::std::println!("{}", #expression);
                }
            }
            HelpHeader::Default(default) => {
                to_tokens! { generator
                    ::std::println!(#default);
                }
            }
        }
    }
}
