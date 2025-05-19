use super::Output;
use proc_macro_util::{Generator, ToTokens};

impl<'a> ToTokens for Output<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            Output::Enum(r#enum) => r#enum.to_tokens(generator),
        }
    }
}
