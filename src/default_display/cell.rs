use crate::DefaultDisplay;
use std::cell::{Ref, RefCell};

pub struct RefCellDisplay<'a, T: DefaultDisplay>(Ref<'a, T>);

impl<'a, T: DefaultDisplay> std::fmt::Display for RefCellDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_display().fmt(f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for RefCell<T> {
    type Display<'a>
        = RefCellDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        RefCellDisplay(self.borrow())
    }
}
