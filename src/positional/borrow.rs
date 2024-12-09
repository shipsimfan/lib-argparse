use crate::{
    Argument, ArgumentSource, DefaultDisplay, Positional, PositionalInfo, PositionalResult, Result,
};
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

    fn sub(
        this: &mut Option<Self>,
        parser: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(|inner| match inner {
            Cow::Owned(inner) => inner,
            Cow::Borrowed(_) => {
                panic!("Cow should never be filled with `Borrowed` before parsing")
            }
        });

        T::sub(&mut inner, parser, command_list)
    }
}
