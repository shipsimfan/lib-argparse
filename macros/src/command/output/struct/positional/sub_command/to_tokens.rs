use super::PositionalSubCommand;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalSubCommand {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalSubCommand {
            index,
            variable_name,
        } = self;

        to_tokens! { generator
            #index => ::argparse::Positional::sub(&mut #variable_name, __source)?,
        }
    }
}
