use crate::{Argument, ArgumentSource, Positional, PositionalInfo, PositionalResult, Result};
use std::sync::{Mutex, RwLock};

impl<T: Positional> Positional for Mutex<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = this.take().map(|inner| Mutex::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Mutex::new(inner.unwrap()));
        result
    }

    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(|inner| Mutex::into_inner(inner).unwrap());
        T::sub(&mut inner, command, source, command_list)
    }
}

impl<T: Positional> Positional for RwLock<T> {
    fn parse<'a>(
        this: &mut Option<Self>,
        argument: Argument<'a>,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult<'a> {
        let mut inner = this.take().map(|inner| RwLock::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(RwLock::new(inner.unwrap()));
        result
    }

    fn sub(
        this: &mut Option<Self>,
        command: Argument,
        source: &mut dyn ArgumentSource,
        command_list: String,
    ) -> Result<bool> {
        let mut inner = this.take().map(|inner| RwLock::into_inner(inner).unwrap());
        T::sub(&mut inner, command, source, command_list)
    }
}
