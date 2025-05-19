use super::VariableDeclaration;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for VariableDeclaration<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let VariableDeclaration { variable_name } = self;

        to_tokens! { generator
            let mut #variable_name = None;
        }
    }
}
