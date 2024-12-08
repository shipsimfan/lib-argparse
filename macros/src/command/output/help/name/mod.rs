use proc_macro_util::tokens::Literal;

mod to_tokens;

/// The name to use when outputing the help flag
pub enum HelpOutputName {
    /// Use the name of the package
    Default,

    /// Use the provided name
    Provided(Literal),
}
