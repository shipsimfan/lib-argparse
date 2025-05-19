use std::borrow::Cow;

use as_f64::AsF64;
use kind::OutputKind;
use proc_macro_util::tokens::Identifier;

mod as_f64;
mod help;
mod kind;
mod r#struct;
mod version;

mod new;
mod to_tokens;

pub use help::{
    FlagGroupHelpOutput, FlagGroupHelpUsageOutput, FlagHelpOutput, FlagHelpUsageOutput, HelpHeader,
    HelpOutput, HelpOutputDescription, HelpOutputName, HelpUsageOutput, PositionalHelpOutput,
    PositionalHelpUsageOutput,
};
pub use r#struct::{
    DefaultValue, Description, FlagGroupDeclaration, FlagGroupLongName, FlagGroupShortName,
    FlagGroupUnwrap, FlagInfo, FlagLongName, FlagShortName, FlagUnwrap, OptionalOutput,
    PositionalInfo, PositionalMatch, PositionalSubCommand, PositionalUnwrap, StructOutput,
    VariableDeclaration,
};
pub use version::VersionOutput;

/// The output code from the Command derive macro
pub struct Output<'a> {
    /// The name of the input item
    name: Cow<'a, Identifier>,

    /// The kind of output to produce
    kind: OutputKind<'a>,
}
