use super::OptionalOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<T: ToTokens> ToTokens for OptionalOutput<T> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            OptionalOutput::None => {
                to_tokens! { generator
                    None
                }
            }
            OptionalOutput::Some(value) => {
                to_tokens! { generator
                    Some(#value)
                }
            }
        }
    }
}
