use proc_macro_util::{
    ast::{Expression, Punctuated},
    to_tokens, Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct Commands<'a> {
    commands: Punctuated<Expression<'a>, Token![,]>,
}

impl<'a> Parse<'a> for Commands<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut commands = Punctuated::new();
        while !parser.empty() {
            commands.push_element(parser.parse()?);

            if parser.peek::<Token![,]>() {
                commands.push_seperator(parser.parse()?);
            }
        }

        Ok(Commands { commands })
    }
}

impl<'a> ToTokens for Commands<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let Commands { commands } = self;

        to_tokens! { generator
            ::argparse::Commands::new(&[#commands])
        }
    }
}
