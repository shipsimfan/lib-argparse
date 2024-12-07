use crate::DefaultDisplay;
use std::borrow::Cow;

impl<'b, T: DefaultDisplay + ToOwned> DefaultDisplay for Cow<'b, T> {
    type Display<'a>
        = T::Display<'a>
    where
        Self: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.as_ref().as_display()
    }
}
