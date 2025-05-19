use crate::{Argument, Positional, PositionalInfo, PositionalResult, Result};

impl<T: Positional> Positional for Option<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = match this.take() {
            Some(inner) => inner,
            None => None,
        };
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(inner);
        result
    }

    fn unwrap(this: Option<Self>, _: &PositionalInfo<Self>) -> Result<Self> {
        Ok(match this {
            Some(this) => this,
            None => None,
        })
    }

    fn is_required(_: &PositionalInfo<Self>) -> bool {
        false
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        T::multiple(&info.drop_default())
    }
}
