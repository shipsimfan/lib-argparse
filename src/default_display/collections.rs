use crate::DefaultDisplay;
use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    fmt::Display,
};

fn fmt_iter<'a, T: 'a + DefaultDisplay, I: Iterator<Item = &'a T>>(
    iter: &mut I,
    f: &mut std::fmt::Formatter,
) -> std::fmt::Result {
    let mut i = 0;
    for value in iter {
        if i > 0 {
            f.write_str(" ")?;
        }

        value.as_display().fmt(f)?;
        i += 1;
    }

    Ok(())
}

pub struct VecDisplay<'a, T: 'a + DefaultDisplay>(&'a [T]);

impl<'a, T: 'a + DefaultDisplay> Display for VecDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iter(&mut self.0.iter(), f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for Vec<T> {
    type Display<'a>
        = VecDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        VecDisplay(&self)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for VecDeque<T> {
    type Display<'a>
        = VecDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        VecDisplay(self.as_slices().0)
    }
}

impl<T: DefaultDisplay, const N: usize> DefaultDisplay for [T; N] {
    type Display<'a>
        = VecDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        VecDisplay(self)
    }
}

pub struct LinkedListDisplay<'a, T: 'a + DefaultDisplay>(&'a LinkedList<T>);

impl<'a, T: 'a + DefaultDisplay> Display for LinkedListDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iter(&mut self.0.iter(), f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for LinkedList<T> {
    type Display<'a>
        = LinkedListDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        LinkedListDisplay(self)
    }
}

pub struct HashSetDisplay<'a, T: 'a + DefaultDisplay>(&'a HashSet<T>);

impl<'a, T: 'a + DefaultDisplay> Display for HashSetDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iter(&mut self.0.iter(), f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for HashSet<T> {
    type Display<'a>
        = HashSetDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        HashSetDisplay(self)
    }
}

pub struct BTreeSetDisplay<'a, T: 'a + DefaultDisplay>(&'a BTreeSet<T>);

impl<'a, T: 'a + DefaultDisplay> Display for BTreeSetDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iter(&mut self.0.iter(), f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for BTreeSet<T> {
    type Display<'a>
        = BTreeSetDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        BTreeSetDisplay(self)
    }
}

pub struct BinaryHeapDisplay<'a, T: 'a + DefaultDisplay>(&'a BinaryHeap<T>);

impl<'a, T: 'a + DefaultDisplay> Display for BinaryHeapDisplay<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_iter(&mut self.0.iter(), f)
    }
}

impl<T: DefaultDisplay> DefaultDisplay for BinaryHeap<T> {
    type Display<'a>
        = BinaryHeapDisplay<'a, T>
    where
        T: 'a;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        BinaryHeapDisplay(self)
    }
}
