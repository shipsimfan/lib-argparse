use super::PositionalInfo;
use crate::command::output::r#struct::{DefaultValue, OptionalOutput};
use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};

impl<'a> PositionalInfo<'a> {
    /// Creates a new [`PositionalInfo`]
    pub fn new(
        info_name: Identifier,
        r#type: Type<'a>,
        value: Literal,
        min_count: Literal,
        max_count: Literal,
        default: OptionalOutput<DefaultValue<'a>>,
    ) -> Self {
        PositionalInfo {
            info_name,
            r#type,
            value,
            min_count,
            max_count,
            default,
        }
    }
}
