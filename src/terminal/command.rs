use crate::Parser;
use std::{borrow::Cow, ops::Deref};

/// An unorded set of commands
pub struct Command<T, E: 'static>(Vec<(Cow<'static, str>, Parser<T, E>)>);

impl<T, E> Command<T, E> {
    /// Creates a new empty `Command` list
    pub fn new() -> Self {
        Command(Vec::new())
    }

    /// Adds a new command into the set
    ///
    /// Returns the old command if there is a name conflict
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

    /// Removes a command with the name `command`
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
