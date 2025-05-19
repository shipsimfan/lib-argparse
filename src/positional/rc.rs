use crate::{Argument, ArgumentSource, Positional, PositionalInfo, PositionalResult, Result};
use std::{rc::Rc, sync::Arc};

impl<T: Positional> Positional for Rc<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = this.take().map(|inner| Rc::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Rc::new(inner.unwrap()));
        result
    }

    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(|inner| Rc::into_inner(inner).unwrap());
        T::sub(&mut inner, command, source, command_list)
    }
}

impl<T: Positional> Positional for Arc<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = this.take().map(|inner| Arc::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Arc::new(inner.unwrap()));
        result
    }

    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(|inner| Arc::into_inner(inner).unwrap());
        T::sub(&mut inner, command, source, command_list)
    }
}
