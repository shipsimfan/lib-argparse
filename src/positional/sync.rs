use crate::{Argument, Positional, PositionalInfo, PositionalResult};
use std::sync::{Mutex, RwLock};

impl<T: Positional> Positional for Mutex<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(|inner| Mutex::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(Mutex::new(inner.unwrap()));
        result
    }
}

impl<T: Positional> Positional for RwLock<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let mut inner = this.take().map(|inner| RwLock::into_inner(inner).unwrap());
        let result = T::parse(&mut inner, argument, &info.drop_default())?;
        *this = Some(RwLock::new(inner.unwrap()));
        result
    }
}
