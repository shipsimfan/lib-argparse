use proc_macro_util::{
    to_tokens,
    tokens::{Equals, Identifier, Literal},
    Generator, Parse, Result, ToTokens, Token,
};

pub struct Parser {
    variable_name: Identifier,
    name: Literal,
    description: Option<Literal>,
}

impl Parse for Parser {
    fn parse(parser: &mut proc_macro_util::Parser) -> Result<Self> {
        let variable_name = parser
            .parse()
            .map_err(|error| error.append("expected a variable name for the parser"))?;

        parser.parse::<Equals>()?;

        let name = parser
            .parse()
            .map_err(|error| error.append("expected a name"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected a description or the end"))?;

        Ok(Parser {
            variable_name,
            name,
            description,
        })
    }
}

impl ToTokens for Parser {
    fn to_tokens(&self, generator: &mut Generator) {
        let Parser {
            variable_name,
            name,
            description,
        } = self;

        to_tokens!(generator
            const #variable_name: ::argparse::Parser = ::argparse::Parser::new().name(#name)
        );

        if let Some(description) = description {
            to_tokens!(generator
                .description(#description)
            );
        }

        Token![;].to_tokens(generator);
    }
}
