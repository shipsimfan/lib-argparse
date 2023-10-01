use std::borrow::Cow;

use crate::{ArgStream, FlagArgument, FlagKind};

mod simple;

pub use simple::SimpleValueParser;

/// Parses a value for a [`ValueFlag`]
pub trait ValueParser: 'static {
    /// An error returned by the parser
    type Error;

    /// The type of object returned by this parser
    type Value;

    /// Parse a value from the argument stream
    ///
    ///  - `args` is the argument stream
    fn parse(&mut self, args: &mut ArgStream) -> Result<Self::Value, crate::Error<Self::Error>>;
}

/// Flag which parses a specific type as it's value
pub struct ValueFlag<T, E: 'static> {
    parser: Box<dyn FnMut(&mut T, &mut ArgStream) -> Result<(), crate::Error<E>>>,
}

impl<T, E> ValueFlag<T, E> {
    /// Creates a new [`ValueFlag`]
    ///
    ///  - `parser` is the [`ValueParser`] which will parse the value
    ///  - `action` is called after the parser to update the options
    ///  - `description` is the description of this argument displayed in the help
    pub fn new<V: ValueParser<Error = E>, S: Into<Cow<'static, str>>>(
        mut parser: V,
        action: impl Fn(&mut T, V::Value) -> Result<(), crate::Error<E>> + 'static,
        description: S,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(
            FlagKind::Value(ValueFlag {
                parser: Box::new(move |options, args| {
                    let value = parser.parse(args)?;
                    action(options, value)
                }),
            }),
            description,
        )
    }

    /// Parses an object
    ///
    ///  - `options` is the developer provided options to be updates
    ///  - `args` is the argument stream parsed from
    pub(crate) fn parse(
        &mut self,
        options: &mut T,
        args: &mut ArgStream,
    ) -> Result<(), crate::Error<E>> {
        (self.parser)(options, args)
    }
}
