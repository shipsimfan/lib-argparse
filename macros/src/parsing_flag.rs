use crate::{
    action_wrapper::ActionWrapper, description::Description, flag_name::FlagName,
    options_type::OptionsType,
};
use proc_macro_util::{
    ast::{Expression, Type, VariableName},
    to_tokens,
    tokens::Literal,
    Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct ParsingFlag<'a> {
    flag_name: FlagName,
    hint: Literal,
    missing: Literal,
    description: Description<'a>,
    options: VariableName,
    options_type: Option<OptionsType>,
    item_mut: Option<Token![mut]>,
    item: VariableName,
    item_type: Type,
    error_indicator: ErrorIndicator,
    action: Expression<'a>,
}

enum ErrorIndicator {
    Present,
    NotPresent,
}

impl<'a> Parse<'a> for ParsingFlag<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser
            .parse()
            .map_err(|error| error.append("expected the flag name"))?;
        let hint = parser
            .parse()
            .map_err(|error| error.append("expected the hint"))?;
        let missing = parser
            .parse()
            .map_err(|error| error.append("expected the missing error message"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected the description"))?;

        parser
            .parse::<Token![|]>()
            .map_err(|error| error.append("expected the action closure"))?;

        let options = parser
            .parse()
            .map_err(|error| error.append("expected the options name"))?;
        let options_type = parser
            .parse()
            .map_err(|error| error.append("expected the options type"))?;

        parser.parse::<Token![,]>()?;

        let item_mut = parser.parse()?;
        let item = parser
            .parse()
            .map_err(|error| error.append("expected the item name"))?;
        parser
            .parse::<Token![:]>()
            .map_err(|error| error.append("expected the item type"))?;
        let item_type = parser
            .parse()
            .map_err(|error| error.append("expected the item type"))?;

        parser.parse::<Token![|]>()?;

        let error_indicator = parser.parse()?;

        let action = parser
            .parse()
            .map_err(|error| error.append("expected the action"))?;

        Ok(ParsingFlag {
            flag_name,
            hint,
            missing,
            description,
            options,
            options_type,
            item_mut,
            item,
            item_type,
            error_indicator,
            action,
        })
    }
}

impl<'a> ToTokens for ParsingFlag<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let ParsingFlag {
            flag_name,
            hint,
            missing,
            description,
            options,
            options_type,
            item_mut,
            item,
            item_type,
            error_indicator,
            action,
        } = self;

        let action = match error_indicator {
            ErrorIndicator::NotPresent => ActionWrapper::Wrap(action),
            ErrorIndicator::Present => ActionWrapper::NoWrap(action),
        };

        to_tokens! { generator
            ::argparse::ParsingFlagArgument::<_, #item_type, _, _, #error_indicator>::new(
                &#missing,
                |#options #options_type, #item_mut #item| #action
            )#flag_name.hint(&#hint).description(#description)
        }
    }
}

impl<'a> Parse<'a> for ErrorIndicator {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(if parser.parse::<Option<Token![?]>>()?.is_some() {
            ErrorIndicator::Present
        } else {
            ErrorIndicator::NotPresent
        })
    }
}

impl ToTokens for ErrorIndicator {
    fn to_tokens(&self, generator: &mut Generator) {
        match self {
            ErrorIndicator::Present => {
                to_tokens! { generator
                    _
                }
            }
            ErrorIndicator::NotPresent => {
                to_tokens! { generator
                    ::core::convert::Infallible
                }
            }
        }
    }
}
