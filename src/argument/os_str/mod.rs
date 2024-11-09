use std::ffi::{OsStr, OsString};

/// An argument which natively is an [`OsStr`]
#[derive(Debug)]
pub enum OsStrArgument<'a> {
    /// This is from a larger source and is borrowed
    Borrowed(&'a OsStr),

    /// This is from a standalone source and is owned
    Owned(OsString),
}
