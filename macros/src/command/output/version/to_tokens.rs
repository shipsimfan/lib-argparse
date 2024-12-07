use super::VersionOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for VersionOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        to_tokens! { generator
            "version" =>
        };

        match self {
            VersionOutput::Default => {
                to_tokens! { generator
                    {
                        ::std::println!("{} v{}", ::std::env!("CARGO_PKG_NAME"), ::std::env!("CARGO_PKG_VERSION"));
                        return Ok(None);
                    }
                }
            }
            VersionOutput::AlternateName(name) => {
                to_tokens! { generator
                    {
                        ::std::println!("{} v{}", #name, ::std::env!("CARGO_PKG_VERSION"));
                        return Ok(None);
                    }
                }
            }
            VersionOutput::UserDefined(expression) => {
                to_tokens! { generator
                    {
                        ::std::println!("{}", #expression);
                        return Ok(None);
                    }
                }
            }
        }
    }
}
