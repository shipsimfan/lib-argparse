use super::FlagGroupInProgress;
use proc_macro_util::{to_tokens, Generator, ToTokens, Token};

impl<'a> ToTokens for FlagGroupInProgress<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagGroupInProgress { first, r#type } = self;

        if !first {
            Token![,]().to_tokens(generator);
        }

        to_tokens! { generator
            <#r#type as ::argparse::FlagGroup>::InProgress
        }
    }
}
