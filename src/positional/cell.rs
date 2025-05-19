use crate::{Argument, ArgumentSource, Positional, PositionalInfo, PositionalResult, Result};
use std::cell::RefCell;

impl<T: Positional> Positional for RefCell<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = this.take().map(RefCell::into_inner);
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(RefCell::new(inner.unwrap()));
        result
    }

    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(RefCell::into_inner);
        T::sub(&mut inner, command, source, command_list)
    }
}
