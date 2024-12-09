use super::{HelpOutput, HelpOutputDescription, HelpOutputName, HelpUsageOutput};

impl<'a> HelpOutput<'a> {
    /// Creates a new [`HelpOutput`]
    pub fn new(
        name: HelpOutputName,
        description: HelpOutputDescription<'a>,
        usage: HelpUsageOutput<'a>,
    ) -> Self {
        HelpOutput {
            name,
            description,
            usage,
        }
    }
}
