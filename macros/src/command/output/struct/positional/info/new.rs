use super::PositionalInfo;
use crate::command::output::{
    as_f64::AsF64,
    r#struct::{DefaultValue, Description, OptionalOutput},
};
use proc_macro_util::{
    ast::{Expression, Type},
    tokens::{Identifier, Literal},
};

impl<'a> PositionalInfo<'a> {
    /// Creates a new [`PositionalInfo`]
    pub fn new(
        info_name: Identifier,
        r#type: Type<'a>,
        value: Literal,
        min: Option<Expression<'a>>,
        max: Option<Expression<'a>>,
        default: OptionalOutput<DefaultValue<'a>>,
        description: OptionalOutput<Description<'a>>,
    ) -> Self {
        PositionalInfo {
            info_name,
            r#type,
            value,
            min: min.map(AsF64::new).into(),
            max: max.map(AsF64::new).into(),
            default,
            description,
        }
    }
}
