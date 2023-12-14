use proc_macro_util::{
    to_tokens, tokens::Literal, Generator, Parse, Parser, Result, ToTokens, Token,
};

pub(crate) struct FlagName {
    short_name: Option<Literal>,
    long_name: Option<Literal>,
}

impl<'a> Parse<'a> for FlagName {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let short_name: Option<Literal> = parser
            .parse()
            .map_err(|error| error.append("expected the short name"))?;

        let long_name = if parser.peek::<Token![,]>() {
            parser.parse::<Token![,]>()?;
            Some(
                parser
                    .parse()
                    .map_err(|error| error.append("expected the long name"))?,
            )
        } else {
            None
        };

        if short_name.is_none() && long_name.is_none() {
            return Err(parser.error("expected a short name or a long name"));
        }

        Ok(FlagName {
            short_name,
            long_name,
        })
    }
}

impl ToTokens for FlagName {
    fn to_tokens(&self, generator: &mut Generator) {
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
    }
}
