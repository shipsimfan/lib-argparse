use super::CommandInfo;

impl<'a> Default for CommandInfo<'a> {
    fn default() -> Self {
        CommandInfo {
            name: None,
            description: None,
            version: None,
            help: false,
            usage_header: None,
        }
    }
}
