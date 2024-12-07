use super::CommandInfo;

impl<'a> Default for CommandInfo<'a> {
    fn default() -> Self {
        CommandInfo {
            name: None,
            version: None,
        }
    }
}
