use super::{
    HelpHeader, HelpOutput, HelpOutputDescription, HelpOutputName, HelpUsageOutput,
    PositionalHelpOutput,
};

impl<'a> HelpOutput<'a> {
    /// Creates a new [`HelpOutput`]
    pub fn new(
        name: HelpOutputName,
        description: HelpOutputDescription<'a>,
        usage: HelpUsageOutput<'a>,
        positional_header: Option<HelpHeader<'a>>,
        positionals: Vec<PositionalHelpOutput>,
    ) -> Self {
        HelpOutput {
            name,
            description,
            usage,
            positional_header,
            positionals,
        }
    }
}
