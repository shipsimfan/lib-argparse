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
            flag_header,
            flags,
            flag_groups,
        } = self;

        to_tokens! { generator
            "help" => {
                ::std::println!("{}", #name);
                (#description)(0);

                #usage

                #positional_header
                #positionals

                #flag_header
                #flags

                #flag_groups

                return Ok(None);
            }
        }
    }
}
