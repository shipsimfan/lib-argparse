use crate::{ArgStream, FlagArgument, FlagKind};

mod simple;

pub use simple::SimpleValueParser;

pub trait ValueParser: 'static {
    type Error;
    type Value;

    fn parse(&mut self, args: &mut ArgStream) -> Result<Self::Value, Self::Error>;
}

pub struct ValueFlag<T, E> {
    parser: Box<dyn FnMut(&mut T, &mut ArgStream) -> Result<(), E>>,
}

impl<T, E> ValueFlag<T, E> {
    pub fn new<V: ValueParser<Error: Into<E>>>(
        mut parser: V,
        action: impl Fn(&mut T, V::Value) -> Result<(), E> + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Value(ValueFlag {
            parser: Box::new(move |options, args| {
                let value = parser.parse(args).map_err(|error| error.into())?;
                action(options, value)
            }),
        }))
    }

    pub(crate) fn parse(&mut self, options: &mut T, args: &mut ArgStream) -> Result<(), E> {
        (self.parser)(options, args)
    }
}
