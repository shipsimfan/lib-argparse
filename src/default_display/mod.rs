mod atomic;
mod borrow;
mod r#box;
mod cell;
mod collections;
mod display;
mod option;
mod os_str;
mod rc;
mod sync;
mod time;
mod tuple;

/// Display a default value
pub trait DefaultDisplay {
    /// The type that displays this value
    type Display<'a>: 'a + Sized + std::fmt::Display
    where
        Self: 'a;

    /// Get the display for this value
    fn as_display<'a>(&'a self) -> Self::Display<'a>;
}
