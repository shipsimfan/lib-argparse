use super::HelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for HelpUsageOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let HelpUsageOutput {
            header,
            positionals,
            flags,
            flag_groups,
        } = self;

        to_tokens! { generator
            #header

            ::std::print!("    {}", __command_list);

            let mut __optional_flags = false;

            #flags
            #flag_groups

            if __optional_flags {
                ::std::print!("[OPTIONS..] ");
            }

            #positionals

            ::std::println!();
        }
    }
}
