use super::EnumVariantDisplay;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for EnumVariantDisplay<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let EnumVariantDisplay {
            name,
            has_field,
            string,
        } = self;

        to_tokens! { generator
            Self::#name
        }

        if has_field {
            to_tokens! { generator
                (_)
            }
        }

        to_tokens! { generator
            => #string,
        }
    }
}
