use super::FlagInfo;
use crate::command::output::{DefaultValue, OptionalOutput};
use proc_macro_util::{
    ast::Type,
    tokens::{Identifier, Literal},
};

impl<'a> FlagInfo<'a> {
    /// Creates a new [`FlagInfo`]
    pub fn new(
        info_name: Identifier,
        r#type: Type<'a>,
        long_name: OptionalOutput<Literal>,
        short_name: OptionalOutput<Literal>,
        value: OptionalOutput<Literal>,
        min_count: Literal,
        max_count: Literal,
        default: OptionalOutput<DefaultValue<'a>>,
    ) -> Self {
        FlagInfo {
            info_name,
            r#type,
            long_name,
            short_name,
            value,
            min_count,
            max_count,
            default,
        }
    }
}
