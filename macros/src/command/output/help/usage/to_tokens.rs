use super::HelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpUsageOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let HelpUsageOutput {
            header,
            positionals,
        } = self;

        match header {
            Some(header) => {
                to_tokens! { generator
                    ::std::println!("{}", #header);
                }
            }
            None => {
                to_tokens! { generator
                    ::std::println!("USAGE:");
                }
            }
        }

        to_tokens! { generator
            let mut __usage = ::std::string::String::new();

            #positionals

            ::std::println!("    {}{}", __command_list, __usage);
            ::std::println!();
        }
    }
}
