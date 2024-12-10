use super::{FlagHelpUsageOutput, HelpUsageOutput, PositionalHelpUsageOutput};
use proc_macro_util::ast::Expression;

impl<'a> HelpUsageOutput<'a> {
    /// Creates a new [`HelpUsageOutput`]
    pub fn new(
        header: Option<Expression<'a>>,
        positionals: Vec<PositionalHelpUsageOutput>,
        flags: Vec<FlagHelpUsageOutput>,
    ) -> Self {
        HelpUsageOutput {
            header,
            positionals,
            flags,
        }
    }
}
