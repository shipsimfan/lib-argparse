use super::FlagGroupDeclaration;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagGroupDeclaration<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupDeclaration {
            variable_name,
            r#type,
        } = self;

        to_tokens! { generator
            let mut #variable_name = <#r#type as ::argparse::FlagGroup>::new_in_progress();
        }
    }
}
