use flags::Flags;
use proc_macro_util::{
    ast::Type,
    to_tokens,
    tokens::{Identifier, Literal},
    Generator, Parse, Result, ToTokens, Token,
};

mod flags;

pub struct Parser<'a> {
    variable_name: Identifier,
    options_type: Type,
    name: Literal,
    description: Option<Literal>,
    flags: Flags<'a>,
}

impl<'a> Parse<'a> for Parser<'a> {
    fn parse(parser: &mut proc_macro_util::Parser<'a>) -> Result<Self> {
        let variable_name = parser
            .parse()
            .map_err(|error| error.append("expected a variable name for the parser"))?;
        parser.parse::<Token![->]>()?;
        let options_type = parser.parse()?;

        let name = parser
            .parse()
            .map_err(|error| error.append("expected a name"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected a description or the end"))?;
        let flags = parser.parse()?;

        Ok(Parser {
            variable_name,
            options_type,
            name,
            description,
            flags,
        })
    }
}

impl<'a> ToTokens for Parser<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let Parser {
            variable_name,
            options_type,
            name,
            description,
            flags,
        } = self;

        to_tokens!(generator
            const #variable_name: ::argparse::Parser<#options_type> = ::argparse::Parser::new().name(#name)
        );

        if let Some(description) = description {
            to_tokens!(generator
                .description(#description)
            );
        }

        flags.to_tokens(generator);

        Token![;]().to_tokens(generator);
    }
}