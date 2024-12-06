use crate::DefaultDisplay;

impl<T: DefaultDisplay> DefaultDisplay for Box<T> {
    type Display<'a>
        = T::Display<'a>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.as_ref().as_display()
    }
}
