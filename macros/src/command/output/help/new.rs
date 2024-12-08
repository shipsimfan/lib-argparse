use super::{HelpOutput, HelpOutputDescription, HelpOutputName};

impl<'a> HelpOutput<'a> {
    /// Creates a new [`HelpOutput`]
    pub fn new(name: HelpOutputName, description: HelpOutputDescription<'a>) -> Self {
        HelpOutput { name, description }
    }
}
