use kind::OutputKind;
use proc_macro_util::tokens::Identifier;

mod help;
mod kind;
mod r#struct;
mod version;

mod new;
mod to_tokens;

pub use help::{
    HelpOutput, HelpOutputDescription, HelpOutputName, HelpUsageOutput, PositionalHelpUsageOutput,
};
pub use r#struct::{
    DefaultValue, FlagInfo, FlagLongName, FlagShortName, FlagUnwrap, OptionalOutput,
    PositionalInfo, PositionalMatch, PositionalSubCommand, PositionalUnwrap, StructOutput,
    VariableDeclaration,
};
pub use version::VersionOutput;

/// The output code from the Command derive macro
pub struct Output<'a> {
    /// The name of the input item
    name: Identifier,

    /// The kind of output to produce
    kind: OutputKind<'a>,
}
