mod from_argument;
mod os_str;
mod str;

mod as_str;
mod display;
mod from;

pub use from_argument::FromArgument;
pub use os_str::OsStrArgument;
pub use str::StrArgument;

/// An arugment from an argument source
#[derive(Debug)]
pub enum Argument<'a> {
    /// The string is natively an [`std::ffi::OsStr`]
    OsStr(OsStrArgument<'a>),

    /// The string is natively a [`str`]
    Str(StrArgument<'a>),
}
