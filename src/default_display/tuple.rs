use crate::DefaultDisplay;

pub struct Tuple2Display<'a, T1: DefaultDisplay, T2: DefaultDisplay>(&'a (T1, T2));

impl<'a, T1: DefaultDisplay, T2: DefaultDisplay> std::fmt::Display for Tuple2Display<'a, T1, T2> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0 .0.as_display(), self.0 .1.as_display())
    }
}

impl<T1: DefaultDisplay, T2: DefaultDisplay> DefaultDisplay for (T1, T2) {
    type Display<'a>
        = Tuple2Display<'a, T1, T2>
    where
        T1: 'a,
        T2: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple2Display(self)
    }
}

pub struct Tuple3Display<'a, T1: DefaultDisplay, T2: DefaultDisplay, T3: DefaultDisplay>(
    &'a (T1, T2, T3),
);

impl<'a, T1: DefaultDisplay, T2: DefaultDisplay, T3: DefaultDisplay> std::fmt::Display
    for Tuple3Display<'a, T1, T2, T3>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display()
        )
    }
}

impl<T1: DefaultDisplay, T2: DefaultDisplay, T3: DefaultDisplay> DefaultDisplay for (T1, T2, T3) {
    type Display<'a>
        = Tuple3Display<'a, T1, T2, T3>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple3Display(self)
    }
}

pub struct Tuple4Display<
    'a,
    T1: DefaultDisplay,
    T2: DefaultDisplay,
    T3: DefaultDisplay,
    T4: DefaultDisplay,
>(&'a (T1, T2, T3, T4));

impl<'a, T1: DefaultDisplay, T2: DefaultDisplay, T3: DefaultDisplay, T4: DefaultDisplay>
    std::fmt::Display for Tuple4Display<'a, T1, T2, T3, T4>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display(),
            self.0 .3.as_display()
        )
    }
}

impl<T1: DefaultDisplay, T2: DefaultDisplay, T3: DefaultDisplay, T4: DefaultDisplay> DefaultDisplay
    for (T1, T2, T3, T4)
{
    type Display<'a>
        = Tuple4Display<'a, T1, T2, T3, T4>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a,
        T4: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple4Display(self)
    }
}

pub struct Tuple5Display<
    'a,
    T1: DefaultDisplay,
    T2: DefaultDisplay,
    T3: DefaultDisplay,
    T4: DefaultDisplay,
    T5: DefaultDisplay,
>(&'a (T1, T2, T3, T4, T5));

impl<
        'a,
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
    > std::fmt::Display for Tuple5Display<'a, T1, T2, T3, T4, T5>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display(),
            self.0 .3.as_display(),
            self.0 .4.as_display()
        )
    }
}

impl<
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
    > DefaultDisplay for (T1, T2, T3, T4, T5)
{
    type Display<'a>
        = Tuple5Display<'a, T1, T2, T3, T4, T5>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a,
        T4: 'a,
        T5: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple5Display(self)
    }
}

pub struct Tuple6Display<
    'a,
    T1: DefaultDisplay,
    T2: DefaultDisplay,
    T3: DefaultDisplay,
    T4: DefaultDisplay,
    T5: DefaultDisplay,
    T6: DefaultDisplay,
>(&'a (T1, T2, T3, T4, T5, T6));

impl<
        'a,
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
    > std::fmt::Display for Tuple6Display<'a, T1, T2, T3, T4, T5, T6>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display(),
            self.0 .3.as_display(),
            self.0 .4.as_display(),
            self.0 .5.as_display()
        )
    }
}

impl<
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
    > DefaultDisplay for (T1, T2, T3, T4, T5, T6)
{
    type Display<'a>
        = Tuple6Display<'a, T1, T2, T3, T4, T5, T6>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a,
        T4: 'a,
        T5: 'a,
        T6: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple6Display(self)
    }
}

pub struct Tuple7Display<
    'a,
    T1: DefaultDisplay,
    T2: DefaultDisplay,
    T3: DefaultDisplay,
    T4: DefaultDisplay,
    T5: DefaultDisplay,
    T6: DefaultDisplay,
    T7: DefaultDisplay,
>(&'a (T1, T2, T3, T4, T5, T6, T7));

impl<
        'a,
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
        T7: DefaultDisplay,
    > std::fmt::Display for Tuple7Display<'a, T1, T2, T3, T4, T5, T6, T7>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display(),
            self.0 .3.as_display(),
            self.0 .4.as_display(),
            self.0 .5.as_display(),
            self.0 .6.as_display()
        )
    }
}

impl<
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
        T7: DefaultDisplay,
    > DefaultDisplay for (T1, T2, T3, T4, T5, T6, T7)
{
    type Display<'a>
        = Tuple7Display<'a, T1, T2, T3, T4, T5, T6, T7>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a,
        T4: 'a,
        T5: 'a,
        T6: 'a,
        T7: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple7Display(self)
    }
}

pub struct Tuple8Display<
    'a,
    T1: DefaultDisplay,
    T2: DefaultDisplay,
    T3: DefaultDisplay,
    T4: DefaultDisplay,
    T5: DefaultDisplay,
    T6: DefaultDisplay,
    T7: DefaultDisplay,
    T8: DefaultDisplay,
>(&'a (T1, T2, T3, T4, T5, T6, T7, T8));

impl<
        'a,
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
        T7: DefaultDisplay,
        T8: DefaultDisplay,
    > std::fmt::Display for Tuple8Display<'a, T1, T2, T3, T4, T5, T6, T7, T8>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {}",
            self.0 .0.as_display(),
            self.0 .1.as_display(),
            self.0 .2.as_display(),
            self.0 .3.as_display(),
            self.0 .4.as_display(),
            self.0 .5.as_display(),
            self.0 .6.as_display(),
            self.0 .7.as_display()
        )
    }
}

impl<
        T1: DefaultDisplay,
        T2: DefaultDisplay,
        T3: DefaultDisplay,
        T4: DefaultDisplay,
        T5: DefaultDisplay,
        T6: DefaultDisplay,
        T7: DefaultDisplay,
        T8: DefaultDisplay,
    > DefaultDisplay for (T1, T2, T3, T4, T5, T6, T7, T8)
{
    type Display<'a>
        = Tuple8Display<'a, T1, T2, T3, T4, T5, T6, T7, T8>
    where
        T1: 'a,
        T2: 'a,
        T3: 'a,
        T4: 'a,
        T5: 'a,
        T6: 'a,
        T7: 'a,
        T8: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        Tuple8Display(self)
    }
}
