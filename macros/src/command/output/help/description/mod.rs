use crate::command::output::Description;

mod to_tokens;

/// The description to use when outputing the help flag
pub enum HelpOutputDescription<'a> {
    /// Use the description from the package
    Default,

    /// Use the provided description
    Provided(Description<'a>),
}
