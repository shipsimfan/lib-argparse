mod command;

pub use command::Command;

pub enum TerminalArgument {
    None,
    Command,
    Positionals,
}
