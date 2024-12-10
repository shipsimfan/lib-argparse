use super::HelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpUsageOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let HelpUsageOutput {
            header,
            positionals,
            flags,
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
            ::std::print!("    {}", __command_list);

            let mut __optional_flags = false;

            #flags

            if __optional_flags {
                ::std::print!("[OPTIONS..] ");
            }

            #positionals

            ::std::println!();
            ::std::println!();
        }
    }
}
