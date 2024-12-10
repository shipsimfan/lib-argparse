use super::CommandInfo;
use crate::command::output::{
    FlagHelpUsageOutput, HelpOutput, HelpOutputDescription, HelpOutputName, HelpUsageOutput,
    PositionalHelpUsageOutput, VersionOutput,
};

impl<'a> CommandInfo<'a> {
    /// Converts this input into a [`VersionOutput`]
    pub fn into_output(
        self,
        positionals: Vec<PositionalHelpUsageOutput>,
        flags: Vec<FlagHelpUsageOutput>,
    ) -> (Option<VersionOutput<'a>>, Option<HelpOutput<'a>>) {
        (
            self.version
                .map(|version| match (self.name.clone(), version) {
                    (_, Some(version)) => VersionOutput::UserDefined(version),
                    (Some(name), None) => VersionOutput::AlternateName(name),
                    (None, None) => VersionOutput::Default,
                }),
            if self.help {
                Some(HelpOutput::new(
                    match self.name {
                        Some(name) => HelpOutputName::Provided(name),
                        None => HelpOutputName::Default,
                    },
                    match self.description {
                        Some(description) => HelpOutputDescription::Provided(description),
                        None => HelpOutputDescription::Default,
                    },
                    HelpUsageOutput::new(self.usage_header, positionals, flags),
                ))
            } else {
                None
            },
        )
    }
}
