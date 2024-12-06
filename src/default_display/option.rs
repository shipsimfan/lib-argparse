use crate::DefaultDisplay;

pub struct OptionDisplay<'a, T: 'a + DefaultDisplay>(Option<&'a T>);

impl<'a, T: 'a + DefaultDisplay> std::fmt::Display for OptionDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(value) => value.as_display().fmt(f),
            None => Ok(()),
        }
    }
}

impl<T: DefaultDisplay> DefaultDisplay for Option<T> {
    type Display<'a>
        = OptionDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        OptionDisplay(self.as_ref())
    }
}
