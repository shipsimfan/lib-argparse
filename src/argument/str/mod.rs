mod deref;
mod display;
mod from;
mod into;
mod to_owned;

/// An argument which natively is a [`str`]
#[derive(Debug, Clone)]
pub enum StrArgument<'a> {
    /// This is from a larger source and is borrowed
    Borrowed(&'a str),

    /// This is from a standalone source and is owned
    Owned(String),
}
