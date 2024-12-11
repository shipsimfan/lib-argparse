use super::Description;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for Description<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let Description { expression } = self;

        to_tokens! { generator
            || { ::std::println!("{}", #expression); }
        }
    }
}
