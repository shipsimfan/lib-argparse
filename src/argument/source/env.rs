use crate::ArgumentSource;
use std::env::{Args, ArgsOs};

impl<'a> ArgumentSource<'a> for Args {
    fn next(&mut self) -> Option<crate::Argument<'a>> {
        Iterator::next(self).map(Into::into)
    }
}

impl<'a> ArgumentSource<'a> for ArgsOs {
    fn next(&mut self) -> Option<crate::Argument<'a>> {
        Iterator::next(self).map(Into::into)
    }
}
