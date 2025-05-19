use super::FlagGroupDeclaration;
use proc_macro_util::{to_tokens, Generator, ToTokens, Token};

impl<'a> ToTokens for FlagGroupDeclaration<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupDeclaration { first, r#type } = self;

        if !first {
            Token![,]().to_tokens(generator);
        }

        to_tokens! { generator
            <#r#type as ::argparse::FlagGroup>::new_in_progress()
        }
    }
}
