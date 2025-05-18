use super::InProgress;
use proc_macro_util::{to_tokens, Generator, ToTokens, Token};

impl<'a> ToTokens for InProgress<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let InProgress { types } = self;

        let mut first = true;
        for r#type in types {
            if first {
                first = false;
            } else {
                Token![,]().to_tokens(generator);
            }

            to_tokens! { generator
                Option<#r#type>
            }
        }
    }
}
