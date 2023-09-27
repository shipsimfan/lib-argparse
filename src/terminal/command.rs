use crate::Parser;
use std::{borrow::Cow, ops::Deref};

pub struct Command<T, E>(Vec<(Cow<'static, str>, Parser<T, E>)>);

impl<T, E> Command<T, E> {
    pub fn new() -> Self {
        Command(Vec::new())
    }

    pub fn add_command<S: Into<Cow<'static, str>>>(
        &mut self,
        command: S,
        parser: Parser<T, E>,
    ) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        let command = command.into();
        let ret = self.remove(&command);
        self.0.push((command, parser));
        ret
    }

    fn remove(&mut self, command: &str) -> Option<(Cow<'static, str>, Parser<T, E>)> {
        for i in 0..self.0.len() {
            if command == self.0[i].0 {
                return Some(self.0.swap_remove(i));
            }
        }

        None
    }
}

impl<T, E> Deref for Command<T, E> {
    type Target = [(Cow<'static, str>, Parser<T, E>)];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
