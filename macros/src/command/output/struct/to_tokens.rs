use super::StructOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for StructOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let StructOutput {} = self;

        to_tokens! { generator
            todo!()
        }
    }
}
