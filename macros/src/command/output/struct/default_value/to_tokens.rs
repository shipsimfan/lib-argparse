use super::DefaultValue;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for DefaultValue<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let DefaultValue { expression } = self;

        to_tokens! { generator
            || #expression
        }
    }
}
