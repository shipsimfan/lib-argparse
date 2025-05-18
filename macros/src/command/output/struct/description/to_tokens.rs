use super::Description;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for Description<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let Description { expressions } = self;

        to_tokens! { generator
            |__count|
        }

        let generator = &mut generator.group_brace();
        let mut first = true;
        for expression in expressions {
            if first {
                first = false;
            } else {
                to_tokens! { generator
                    ::std::println!();
                    for _ in 0..__count {
                        ::std::print!(" ");
                    }
                }
            }

            to_tokens! { generator
                ::std::print!("{}", #expression);
            }
        }
    }
}
