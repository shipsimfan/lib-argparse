use crate::{Argument, Positional, PositionalInfo, PositionalResult};

impl<T: Positional> Positional for Box<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(Box::into_inner);
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Box::new(inner.unwrap()));
        result
    }
}
