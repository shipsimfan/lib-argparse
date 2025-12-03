use as_f64::AsF64;

mod as_f64;
mod help;
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
pub enum Output<'a> {
    /// Produce the output for a struct
    Struct(StructOutput<'a>),
}
