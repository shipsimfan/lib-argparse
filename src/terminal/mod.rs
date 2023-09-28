mod command;
mod positional;

pub use command::Command;
pub use positional::{
    CollectOsPositionalParser, CollectPositionalParser, PositionalParser, Positionals,
    SimplePositionalParser,
};

pub enum TerminalArgument<T, E> {
    None,
    Command(Command<T, E>),
    Positionals(Positionals<T, E>),
}
