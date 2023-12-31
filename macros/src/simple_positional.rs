use crate::{action_wrapper::ActionWrapper, description::Description, options_type::OptionsType};
use proc_macro_util::{
    ast::{Expression, VariableName},
    to_tokens,
    tokens::Literal,
    Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct SimplePositional<'a> {
    name: Literal,
    count: Literal,
    hint: Literal,
    description: Description<'a>,
    options: VariableName,
    options_type: Option<OptionsType>,
    index: VariableName,
    parameters_mut: Option<Token![mut]>,
    parameters: VariableName,
    error_indicator: Option<Token![?]>,
    action: Expression<'a>,
}

impl<'a> Parse<'a> for SimplePositional<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let name = parser
            .parse()
            .map_err(|error| error.append("expected the name"))?;
        let count = parser
            .parse()
            .map_err(|error| error.append("expected the count"))?;
        parser.parse::<Token![*]>()?;
        let hint = parser
            .parse()
            .map_err(|error| error.append("expected the hint"))?;
        let description = parser
            .parse()
            .map_err(|error| error.append("expected the description"))?;

        parser
            .parse::<Token![|]>()
            .map_err(|error| error.append("expected the action closure"))?;
        let options = parser
            .parse()
            .map_err(|error| error.append("expected the options variable"))?;

        let options_type = parser
            .parse()
            .map_err(|error| error.append("expected the options type"))?;

        parser.parse::<Token![,]>()?;

        let index = parser
            .parse()
            .map_err(|error| error.append("expected the index variable"))?;

        parser.parse::<Token![,]>()?;

        let parameters_mut = parser.parse()?;
        let parameters = parser
            .parse()
            .map_err(|error| error.append("expected the parameters"))?;
        parser
            .parse::<Token![|]>()
            .map_err(|error| error.append("expected the end of the parameters"))?;

        let error_indicator = parser
            .parse()
            .map_err(|error| error.append("expected the error indicator"))?;

        let action = parser
            .parse()
            .map_err(|error| error.append("expected the action"))?;

        Ok(SimplePositional {
            name,
            count,
            hint,
            description,
            options,
            options_type,
            index,
            parameters,
            parameters_mut,
            action,
            error_indicator,
        })
    }
}

impl<'a> ToTokens for SimplePositional<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let SimplePositional {
            name,
            count,
            hint,
            description,
            options,
            options_type,
            index,
            parameters_mut,
            parameters,
            error_indicator,
            action,
        } = self;

        let action = if error_indicator.is_some() {
            ActionWrapper::NoWrap(action)
        } else {
            ActionWrapper::Wrap(action)
        };

        to_tokens! { generator
            ::argparse::SimplePositionalArgument
        }

        if error_indicator.is_none() {
            to_tokens! { generator
                ::<_, ::core::convert::Infallible, _>
            }
        }

        to_tokens! { generator
            ::new(&#name, unsafe { ::std::num::NonZeroUsize::new_unchecked(#count) }, |#options #options_type, #index, #parameters_mut #parameters| #action).description(#description).hint(&#hint)
        }
    }
}
