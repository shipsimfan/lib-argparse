use super::Output;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for Output<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let Output { name, kind } = self;

        to_tokens! { generator
            impl ::argparse::Command for #name {
                fn parse(__source: &mut dyn ::argparse::ArgumentSource) -> ::argparse::Result<Self> {
                    #kind
                }
            }
        }
    }
}
