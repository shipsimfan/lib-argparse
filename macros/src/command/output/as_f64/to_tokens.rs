use super::AsF64;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<T: ToTokens> ToTokens for AsF64<T> {
    fn to_tokens(self, generator: &mut Generator) {
        let AsF64 { value } = self;

        to_tokens! {generator
            #value as f64
        }
    }
}
