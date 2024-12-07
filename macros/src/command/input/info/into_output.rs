use super::CommandInfo;
use crate::command::output::VersionOutput;

impl<'a> CommandInfo<'a> {
    /// Converts this input into a [`VersionOutput`]
    pub fn into_output(self) -> Option<VersionOutput<'a>> {
        self.version.map(|version| match (self.name, version) {
            (_, Some(version)) => VersionOutput::UserDefined(version),
            (Some(name), None) => VersionOutput::AlternateName(name),
            (None, None) => VersionOutput::Default,
        })
    }
}
