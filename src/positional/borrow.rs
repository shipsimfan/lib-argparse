use crate::{Argument, DefaultDisplay, Positional, PositionalInfo, PositionalResult};
use std::borrow::Cow;

impl<'a, T: Positional, B: DefaultDisplay + ToOwned<Owned = T>> Positional for Cow<'a, B> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(|inner| match inner {
            Cow::Owned(inner) => inner,
            Cow::Borrowed(_) => {
                panic!("Cow should never be filled with `Borrowed` before parsing")
            }
        });
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Cow::Owned(inner.unwrap()));
        result
    }
}
