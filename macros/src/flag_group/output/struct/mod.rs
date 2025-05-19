use crate::command::{FlagHelpOutput, FlagHelpUsageOutput, FlagInfo};
use proc_macro_util::tokens::Identifier;
use std::borrow::Cow;

mod in_progress;
mod long_name;
mod new_in_progress;
mod short_name;
mod unwrap;

mod new;
mod to_tokens;

pub use in_progress::InProgress;
pub use long_name::FlagLongName;
pub use new_in_progress::NewInProgress;
pub use short_name::FlagShortName;
pub use unwrap::FlagUnwrap;

/// The output code for a struct
pub struct StructOutput<'a> {
    /// The name of the struct
    name: Cow<'a, Identifier>,

    /// The name of the module to produce the implementation in
    module_name: Identifier,

    /// The info describing the flags
    infos: Vec<FlagInfo<'a>>,

    /// The types in the in-progress tuple
    in_progress: InProgress<'a>,

    /// Creates a new empty in-progress tuple
    new_in_progress: NewInProgress,

    /// Match arms for flag long names
    long_names: Vec<FlagLongName>,

    /// Match arms for flag short names
    short_names: Vec<FlagShortName>,

    /// Unwrapping of flags
    unwraps: Vec<FlagUnwrap<'a>>,

    /// Prints the usages for the flags in the group
    usages: Vec<FlagHelpUsageOutput>,

    /// The help displays for the contained flags
    helps: Vec<FlagHelpOutput>,
}
