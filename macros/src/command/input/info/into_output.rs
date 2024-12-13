use super::CommandInfo;
use crate::command::output::{
    FlagHelpOutput, FlagHelpUsageOutput, HelpHeader, HelpOutput, HelpOutputDescription,
    HelpOutputName, HelpUsageOutput, PositionalHelpOutput, PositionalHelpUsageOutput,
    VersionOutput,
};

impl<'a> CommandInfo<'a> {
    /// Converts this input into a [`VersionOutput`]
    pub fn into_output(
        self,
        positional_usages: Vec<PositionalHelpUsageOutput>,
        positional_help: Vec<PositionalHelpOutput>,
        flag_usages: Vec<FlagHelpUsageOutput>,
        flag_help: Vec<FlagHelpOutput>,
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
                    HelpUsageOutput::new(
                        match self.usage_header {
                            Some(expression) => HelpHeader::UserProvided(expression),
                            None => HelpHeader::Default("USAGE:"),
                        },
                        positional_usages,
                        flag_usages,
                    ),
                    if positional_help.len() > 0 {
                        Some(match self.positional_header {
                            Some(expression) => HelpHeader::UserProvided(expression),
                            None => HelpHeader::Default("ARGUMENTS:"),
                        })
                    } else {
                        None
                    },
                    positional_help,
                    if flag_help.len() > 0 {
                        Some(match self.flag_header {
                            Some(expression) => HelpHeader::UserProvided(expression),
                            None => HelpHeader::Default("OPTIONS:"),
                        })
                    } else {
                        None
                    },
                    flag_help,
                ))
            } else {
                None
            },
        )
    }
}
