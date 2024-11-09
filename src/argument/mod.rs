mod os_str;
mod str;

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
