use crate::{
    action_wrapper::ActionWrapper, description::Description, flag_name::FlagName,
    options_type::OptionsType,
};
use parameter_info::ParameterInfo;
use proc_macro::Span;
use proc_macro_util::{
    ast::{Expression, VariableName},
    to_tokens,
    tokens::Literal,
    Generator, Parse, Parser, Result, ToTokens, Token,
};

mod parameter_info;

pub struct SimpleFlag<'a> {
    flag_name: FlagName,
    parameter_info: Option<ParameterInfo<'a>>,
    description: Description<'a>,
    options: VariableName,
    options_type: Option<OptionsType>,
    parameters_mut: Option<Token![mut]>,
    parameters: VariableName,
    error_indicator: Option<Token![?]>,
    action: Expression<'a>,
}

impl<'a> Parse<'a> for SimpleFlag<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser
            .parse()
            .map_err(|error| error.append("expected the flag name"))?;
        let parameter_info = parser
            .parse()
            .map_err(|error| error.append("expected the paramter info"))?;
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

        Ok(SimpleFlag {
            flag_name,
            parameter_info,
            description,
            options,
            options_type,
            parameters,
            parameters_mut,
            action,
            error_indicator,
        })
    }
}

impl<'a> ToTokens for SimpleFlag<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let SimpleFlag {
            flag_name,
            parameter_info,
            description,
            options,
            options_type,
            parameters,
            parameters_mut,
            error_indicator,
            action,
        } = self;

        let action = if error_indicator.is_some() {
            ActionWrapper::NoWrap(action)
        } else {
            ActionWrapper::Wrap(action)
        };

        let hint = parameter_info
            .as_ref()
            .map(|parameter_info| parameter_info.hint());

        let zero_count = Literal::new_usize_unsuffixed(0, Span::call_site());
        let zero_string = Expression::Literal(Literal::new_string("", Span::call_site()));

        let (count, missing) = parameter_info
            .as_ref()
            .map(|parameter_info| (parameter_info.count(), parameter_info.missing()))
            .unwrap_or((&zero_count, &zero_string));

        to_tokens! { generator
            ::argparse::SimpleFlagArgument
        }

        if error_indicator.is_none() {
            to_tokens! { generator
                ::<_, ::core::convert::Infallible, _>
            }
        }

        to_tokens! { generator
            ::new(#count, &#missing, |#options #options_type, #parameters_mut #parameters| #action)#flag_name.description(#description)
        }

        if let Some(hint) = hint {
            to_tokens! { generator
                .hint(&#hint)
            }
        }
    }
}
