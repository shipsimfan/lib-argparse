use super::HelpOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let HelpOutput {
            name,
            description,
            usage,
            positional_header,
            positionals,
        } = self;

        to_tokens! { generator
            "help" => {
                ::std::println!("{}", #name);
                ::std::println!("{}", #description);

                #usage

                #positional_header
                #positionals

                return Ok(None);
            }
        }
    }
}
