mod os_str;
mod source;
mod str;

mod as_str;
mod display;
mod from;

pub use os_str::OsStrArgument;
pub use source::{ArgsOsSource, ArgsSource, ArgumentSource};
pub use str::StrArgument;

/// An arugment from an argument source
#[derive(Debug, Clone)]
pub enum Argument<'a> {
    /// The string is natively an [`std::ffi::OsStr`]
    OsStr(OsStrArgument<'a>),

    /// The string is natively a [`str`]
    Str(StrArgument<'a>),
}
