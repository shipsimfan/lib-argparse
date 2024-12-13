use super::FlagInfo;
use crate::command::output::{DefaultValue, Description, OptionalOutput};
use proc_macro_util::{
    ast::Type,
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
        min_count: Literal,
        max_count: Literal,
        default: OptionalOutput<DefaultValue<'a>>,
        description: OptionalOutput<Description<'a>>,
    ) -> Self {
        FlagInfo {
            info_name,
            r#type,
            info_long_name,
            info_short_name,
            value,
            min_count,
            max_count,
            default,
            description,
        }
    }
}
