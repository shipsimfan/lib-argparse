use std::ffi::{OsStr, OsString};

mod deref;
mod display;
mod from;
mod into;
mod to_owned;
mod to_str;

/// An argument which natively is an [`OsStr`]
#[derive(Debug)]
pub enum OsStrArgument<'a> {
    /// This is from a larger source and is borrowed
    Borrowed(&'a OsStr),

    /// This is from a standalone source and is owned
    Owned(OsString),
}
