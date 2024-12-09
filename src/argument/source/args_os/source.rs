use crate::{ArgsOsSource, Argument, ArgumentSource};

impl<'a> ArgumentSource<'a> for ArgsOsSource {
    fn next(&mut self) -> Option<Argument<'a>> {
        self.args.next().map(Into::into)
    }

    fn empty(&self) -> bool {
        self.args.len() == 0
    }
}
