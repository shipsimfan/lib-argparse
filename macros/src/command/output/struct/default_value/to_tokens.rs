use super::DefaultValue;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for DefaultValue<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            DefaultValue::None => {
                to_tokens! { generator
                    None
                }
            }
            DefaultValue::Some(expression) => {
                to_tokens! { generator
                    Some(|| #expression)
                }
            }
        }
    }
}
