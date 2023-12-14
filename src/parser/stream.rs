use std::ffi::OsString;

/// A stream of arguments
pub(super) enum ArgumentStream<'a> {
    /// The stream is a source of [`String`]s
    String(&'a mut dyn Iterator<Item = String>),

    /// The stream is a source of [`OsString`]s
    OsString(&'a mut dyn Iterator<Item = OsString>),
}

impl<'a> ArgumentStream<'a> {
    /// Creates a new [`ArgumentStream`] with iterator producing [`String`]s
    ///
    /// ## Parameters
    ///  * `iter` - The iterator of [`String`]s
    ///
    /// ## Return Value
    /// Returns the newly created [`ArgumentStream`]
    pub(super) fn new(iter: &'a mut dyn Iterator<Item = String>) -> Self {
        ArgumentStream::String(iter)
    }

    /// Creates a new [`ArgumentStream`] with iterator producing [`OsString`]s
    ///
    /// ## Parameters
    ///  * `iter` - The iterator of [`OsString`]s
    ///
    /// ## Return Value
    /// Returns the newly created [`ArgumentStream`]
    pub(super) fn new_os(iter: &'a mut dyn Iterator<Item = OsString>) -> Self {
        ArgumentStream::OsString(iter)
    }

    /// Gets the next [`OsString`] in the stream
    ///
    /// ## Return Value
    /// Returns the next [`OsString`] in the stream if it exists.
    pub(super) fn next_os(&mut self) -> Option<OsString> {
        match self {
            ArgumentStream::OsString(iter) => iter.next(),
            ArgumentStream::String(iter) => iter.next().map(OsString::from),
        }
    }
}
