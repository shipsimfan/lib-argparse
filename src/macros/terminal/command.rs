use crate::{Command, Parser, TerminalArgument};
use std::borrow::Cow;

#[macro_export]
/// Creates a [`Command`]
///
///  - `command_name` is the usage hint. It is also the name of the command placed at the "$" in the unknown error string "unknown $"
///  - `command` is the name of one command
///  - `help` is the help message for a command
///  - `parser` is the parser returned on matching the command
macro_rules! command {
    ($command_name: expr) => {
        $crate::macros::__command($command_name.into(), ::std::vec::Vec::new())
    };

    ($command_name: expr, [
        $($command: literal $help: literal => $parser: expr),*
    ]) => {
        $crate::macros::__command($command_name.into(), ::std::vec![
            $(($command.into(), $help.into(), $parser, ::std::option::Option::None)),*
        ])
    };

    ($command_name: expr, [
        $($command: literal $help: literal (|$options: ident| $action: tt) => $parser: expr),*
    ]) => {
        $crate::macros::__command($command_name.into(), ::std::vec![
            $(($command.into(), $help.into(), $parser, ::std::option::Option::Some(::std::boxed::Box::new(|$options| $action)))),*
        ])
    };
}

#[doc(hidden)]
/// Creates a [`Command`]
///
///  - `command_name` is the usage hint. It is also the name of the command placed at the "$" in the unknown error string "unknown $"
///  - `commands` is the list of commands
pub fn __command<T: 'static, E>(
    command_name: Cow<'static, str>,
    commands: Vec<(
        Cow<'static, str>,
        Cow<'static, str>,
        Parser<T, E>,
        Option<Box<dyn Fn(&mut T)>>,
    )>,
) -> TerminalArgument<T, E> {
    let mut command = Command::new(command_name);

    for (command_name, help_message, parser, action) in commands {
        let result = if let Some(action) = action {
            command.add_command_action(command_name, help_message, parser, action)
        } else {
            command.add_command(command_name, help_message, parser)
        };
        assert!(result.is_none(), "Command is repeated");
    }

    TerminalArgument::Command(command)
}
