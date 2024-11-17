use crate::{ArgsSource, Argument, ArgumentSource};

impl<'a> ArgumentSource<'a> for ArgsSource<'a> {
    fn next(&mut self) -> Option<Argument<'a>> {
        self.args.next().map(Into::into)
    }

    fn empty(&self) -> bool {
        self.args.len() == 0
    }

    fn program_name(&self) -> Option<&Argument<'a>> {
        Some(&self.first)
    }
}
