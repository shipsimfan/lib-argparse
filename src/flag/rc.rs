use crate::{ArgumentSource, Flag, FlagInfo, Result};
use std::{rc::Rc, sync::Arc};

impl<T: Flag> Flag for Rc<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Rc::new)
    }
}

impl<T: Flag> Flag for Arc<T> {
    fn parse(source: &mut dyn ArgumentSource, info: &FlagInfo<Self>, long: bool) -> Result<Self> {
        T::parse(source, &info.drop_default(), long).map(Arc::new)
    }

    fn takes_value(info: &FlagInfo<Self>) -> bool {
        T::takes_value(&info.drop_default())
    }
}
