use crate::DefaultDisplay;
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard};

pub struct MutexDisplay<'a, T: DefaultDisplay>(MutexGuard<'a, T>);

impl<'a, T: DefaultDisplay> std::fmt::Display for MutexDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_display().fmt(f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for Mutex<T> {
    type Display<'a>
        = MutexDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        MutexDisplay(self.lock().unwrap())
    }
}

pub struct RwLockDisplay<'a, T: DefaultDisplay>(RwLockReadGuard<'a, T>);

impl<'a, T: DefaultDisplay> std::fmt::Display for RwLockDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.as_display().fmt(f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for RwLock<T> {
    type Display<'a>
        = RwLockDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        RwLockDisplay(self.read().unwrap())
    }
}
