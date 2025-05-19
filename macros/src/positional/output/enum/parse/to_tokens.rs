use super::EnumVariantParse;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for EnumVariantParse<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let EnumVariantParse { string, name } = self;

        to_tokens! { generator
            #string =>
        }

        match name {
            Some(name) => {
                to_tokens! { generator
                    {
                        *this = Some(Self::#name);
                        ::argparse::PositionalResult::Next
                    }
                }
            }
            None => {
                to_tokens! { generator
                    ::argparse::PositionalResult::Sub(argument),
                }
            }
        }
    }
}
