use crate::{
    action_wrapper::ActionWrapper, description::Description, options_type::OptionsType,
    parser::Flags,
};
use proc_macro::Delimiter;
use proc_macro_util::{
    ast::{Expression, VariableName},
    to_tokens,
    tokens::{Group, Literal},
    Error, Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct Command<'a> {
    name: Literal,
    description: Description<'a>,
    options: VariableName,
    options_type: Option<OptionsType>,
    error_indicator: Option<Token![?]>,
    action: Expression<'a>,
    command_parser: CommandParser<'a>,
}

struct CommandParser<'a> {
    description: Option<Description<'a>>,
    parser_name: Literal,
    terminal: Option<Expression<'a>>,
    flags: Flags<'a>,
}

impl<'a> Parse<'a> for Command<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser
            .parse()
            .map_err(|error| error.append("expected the command name"))?;
        let description: Description = parser
            .parse()
            .map_err(|error| error.append("expected the description"))?;

        parser
            .parse::<Token![|]>()
            .map_err(|error| error.append("expected a closure"))?;
        let options = parser
            .parse()
            .map_err(|error| error.append("expected the options variable"))?;
        let options_type = parser.parse()?;
        parser
            .parse::<Token![|]>()
            .map_err(|error| error.append("expected the end of the closure"))?;
        let error_indicator = parser.parse()?;
        let action = parser
            .parse()
            .map_err(|error| error.append("expected the action"))?;

        let mut command_parser: CommandParser = parser.parse()?;
        command_parser.description = Some(description.clone());

        Ok(Command {
            name,
            description,
            options,
            options_type,
            error_indicator,
            action,
            command_parser,
        })
    }
}

impl<'a> ToTokens for Command<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let Command {
            name,
            description,
            options,
            options_type,
            error_indicator,
            action,
            command_parser,
        } = self;

        let action = if error_indicator.is_some() {
            ActionWrapper::NoWrap(action)
        } else {
            ActionWrapper::Wrap(action)
        };

        to_tokens! { generator
            ::argparse::Command::new(
                #name,
                &|#options #options_type| #action,
                &#command_parser
            ).description(#description)
        }
    }
}

impl<'a> Parse<'a> for CommandParser<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let group = parser
            .parse::<Group>()
            .map_err(|error| error.append("expected the parser info"))?;
        if group.delimiter() != Delimiter::Brace {
            return Err(Error::new_at("expected curly braces", group.span()));
        }

        let mut parser = group.tokens();
        let parser_name = parser
            .parse()
            .map_err(|error| error.append("expected the parser name"))?;
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
            .map_err(|error| error.append("expected the flags"))?;

        Ok(CommandParser {
            description: None,
            parser_name,
            terminal,
            flags,
        })
    }
}

impl<'a> ToTokens for CommandParser<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let CommandParser {
            description,
            parser_name,
            terminal,
            flags,
        } = self;

        to_tokens! { generator
            ::argparse::Parser::new()
                    .name(&#parser_name)
        }

        if let Some(description) = description {
            to_tokens! { generator
                .description(#description)
            }
        }

        if let Some(terminal) = terminal {
            to_tokens! { generator
                .terminal(&#terminal)
            }
        }

        flags.to_tokens(generator);
    }
}
