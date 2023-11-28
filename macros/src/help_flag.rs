use proc_macro_util::{to_tokens, tokens::Literal, Parse, ToTokens, Token};

pub struct HelpFlag {
    short_name: Option<Literal>,
    long_name: Option<Literal>,
    exit: bool,
}

impl<'a> Parse<'a> for HelpFlag {
    fn parse(parser: &mut proc_macro_util::Parser<'a>) -> proc_macro_util::Result<Self> {
        let short_name = parser.parse()?;

        let long_name = if parser.peek::<Token![,]>() {
            parser.parse::<Token![,]>()?;
            Some(parser.parse()?)
        } else {
            None
        };

        let exit = if let Some(identifier) = parser.identifier() {
            if identifier != "noexit" {
                return Err(parser.error("expected `noexit` or the end"));
            }

            false
        } else {
            true
        };

        Ok(HelpFlag {
            short_name,
            long_name,
            exit,
        })
    }
}

impl ToTokens for HelpFlag {
    fn to_tokens(&self, generator: &mut proc_macro_util::Generator) {
        to_tokens! { generator
            ::argparse::HelpFlagArgument::new()
        };

        if let Some(short_name) = &self.short_name {
            to_tokens! { generator
                .short_name(#short_name)
            }
        }

        if let Some(long_name) = &self.long_name {
            to_tokens! { generator
                .long_name(#long_name)
            }
        }

        if !self.exit {
            to_tokens! { generator
                .set_no_exit()
            }
        }
    }
}
