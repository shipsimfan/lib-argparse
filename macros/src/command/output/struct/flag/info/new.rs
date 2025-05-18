use super::FlagInfo;
use crate::command::output::{as_f64::AsF64, DefaultValue, Description, OptionalOutput};
use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

impl<'a> FlagInfo<'a> {
    /// Creates a new [`FlagInfo`]
    pub fn new(
        info_name: Identifier,
        r#type: Type<'a>,
        info_long_name: OptionalOutput<Literal>,
        info_short_name: OptionalOutput<Literal>,
        value: OptionalOutput<Literal>,
        min: Option<Expression<'a>>,
        max: Option<Expression<'a>>,
        default: OptionalOutput<DefaultValue<'a>>,
        description: OptionalOutput<Description<'a>>,
    ) -> Self {
        FlagInfo {
            info_name,
            r#type,
            info_long_name,
            info_short_name,
            value,
            min: min.map(AsF64::new).into(),
            max: max.map(AsF64::new).into(),
            default,
            description,
        }
    }
}
