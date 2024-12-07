use crate::{Argument, Positional, PositionalInfo, PositionalResult};
use std::{rc::Rc, sync::Arc};

impl<T: Positional> Positional for Rc<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(|inner| Rc::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Rc::new(inner.unwrap()));
        result
    }
}

impl<T: Positional> Positional for Arc<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(|inner| Arc::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Arc::new(inner.unwrap()));
        result
    }
}
