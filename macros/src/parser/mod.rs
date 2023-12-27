use flags::Flags;
use proc_macro_util::{
    ast::{Expression, Type},
    to_tokens,
    tokens::{Group, Identifier, Literal},
    Generator, Parse, Result, ToTokens, Token,
};

mod flags;

pub struct Parser<'a> {
    variable_name: Identifier,
    options_type: Type,
    name: Literal,
    description: Option<Literal>,
    terminal: Option<Expression<'a>>,
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

        let terminal = if !parser.peek::<Group>() {
            Some(parser.parse()?)
        } else {
            None
        };

        let flags = parser
            .parse()
            .map_err(|error| error.append("expected flags"))?;

        Ok(Parser {
            variable_name,
            options_type,
            name,
            description,
            terminal,
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
            terminal,
            flags,
        } = self;

        to_tokens! { generator
            const #variable_name: ::argparse::Parser<#options_type> = ::argparse::Parser::new().name(&#name)
        }

        if let Some(description) = description {
            to_tokens! { generator
                .description(&#description)
            }
        }

        if let Some(terminal) = terminal {
            to_tokens! { generator
                .terminal(&#terminal)
            }
        }

        flags.to_tokens(generator);

        Token![;]().to_tokens(generator);
    }
}
