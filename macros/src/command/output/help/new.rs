use super::{
    flag_group::FlagGroupHelpOutput, FlagHelpOutput, HelpHeader, HelpOutput, HelpOutputDescription,
    HelpOutputName, HelpUsageOutput, PositionalHelpOutput,
};

impl<'a> HelpOutput<'a> {
    /// Creates a new [`HelpOutput`]
    pub fn new(
        name: HelpOutputName,
        description: HelpOutputDescription<'a>,
        usage: HelpUsageOutput<'a>,
        positional_header: Option<HelpHeader<'a>>,
        positionals: Vec<PositionalHelpOutput>,
        flag_header: Option<HelpHeader<'a>>,
        flags: Vec<FlagHelpOutput>,
        flag_groups: Vec<FlagGroupHelpOutput<'a>>,
    ) -> Self {
        HelpOutput {
            name,
            description,
            usage,
            positional_header,
            positionals,
            flag_header,
            flags,
            flag_groups,
        }
    }
}
