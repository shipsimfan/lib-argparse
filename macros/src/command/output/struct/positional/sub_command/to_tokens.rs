use super::PositionalSubCommand;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for PositionalSubCommand<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalSubCommand {
            index,
            variable_name,
        } = self;

        to_tokens! { generator
            #index => match ::argparse::Positional::sub(
                &mut #variable_name,
                __command,
                __source,
                ::std::format!("{}{} ", __command_list, __argument)
            )? {
                true => break,
                false => return Ok(None),
            },
        }
    }
}
