use super::NewInProgress;
use proc_macro_util::{to_tokens, Generator, ToTokens, Token};

impl ToTokens for NewInProgress {
    fn to_tokens(self, generator: &mut Generator) {
        let NewInProgress { count } = self;

        for i in 0..count {
            if i > 0 {
                Token![,]().to_tokens(generator);
            }

            to_tokens! { generator
                None
            }
        }
    }
}
