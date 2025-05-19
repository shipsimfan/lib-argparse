use super::FlagGroupHelpUsageOutput;
use proc_macro_util::ast::Type;

impl<'a> FlagGroupHelpUsageOutput<'a> {
    /// Creates a new [`FlagGroupHelpUsageOutput`]
    pub fn new(r#type: Type<'a>) -> Self {
        FlagGroupHelpUsageOutput { r#type }
    }
}
