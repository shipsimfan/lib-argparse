use crate::DefaultDisplay;
use std::{rc::Rc, sync::Arc};

impl<T: DefaultDisplay> DefaultDisplay for Rc<T> {
    type Display<'a>
        = T::Display<'a>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.as_ref().as_display()
    }
}

impl<T: DefaultDisplay> DefaultDisplay for Arc<T> {
    type Display<'a>
        = T::Display<'a>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.as_ref().as_display()
    }
}
