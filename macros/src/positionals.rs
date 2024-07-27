use proc_macro_util::{
    ast::{Expression, Punctuated},
    to_tokens, Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct Positionals<'a> {
    positionals: Punctuated<Positional<'a>, Token![,]>,
}

struct Positional<'a>(Expression<'a>);

impl<'a> Parse<'a> for Positionals<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut positionals = Punctuated::new();
        while !parser.empty() {
            positionals.push_element(
                parser
                    .parse()
                    .map_err(|error| error.append("expected a positional argument"))?,
            );

            if parser.peek::<Token![,]>() {
                positionals.push_separator(parser.parse()?);
            }
        }

        Ok(Positionals { positionals })
    }
}

impl<'a> ToTokens for Positionals<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let Positionals { positionals } = self;

        to_tokens! { generator
            ::argparse::PositionalTerminalArgument::new().positionals(&[#positionals])
        }
    }
}

impl<'a> Parse<'a> for Positional<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse().map(|expression| Positional(expression))
    }
}

impl<'a> ToTokens for Positional<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let Positional(expression) = self;

        to_tokens! { generator
            &#expression
        }
    }
}
