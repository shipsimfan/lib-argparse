use crate::description::Description;
use proc_macro_util::{
    ast::{Expression, Type, Visibility},
    to_tokens,
    tokens::{Group, Identifier, Literal},
    Generator, Parse, Result, ToTokens, Token,
};

mod flags;

pub use flags::Flags;

pub struct Parser<'a> {
    visibility: Option<Visibility<'a>>,
    variable_name: Identifier,
    options_type: Type,
    name: Literal,
    description: Option<Description<'a>>,
    terminal: Option<Expression<'a>>,
    flags: Flags<'a>,
}

impl<'a> Parse<'a> for Parser<'a> {
    fn parse(parser: &mut proc_macro_util::Parser<'a>) -> Result<Self> {
        let visibility = parser.parse()?;
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

        let terminal = if !parser.empty() && !parser.peek::<Group>() {
            Some(
                parser
                    .parse()
                    .map_err(|error| error.append("expected the terminal argument"))?,
            )
        } else {
            None
        };

        let flags = parser
            .parse()
            .map_err(|error| error.append("expected flags"))?;

        Ok(Parser {
            visibility,
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
            visibility,
            variable_name,
            options_type,
            name,
            description,
            terminal,
            flags,
        } = self;

        to_tokens! { generator
            #visibility const #variable_name: ::argparse::Parser<#options_type> = ::argparse::Parser::new().name(&#name)
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
