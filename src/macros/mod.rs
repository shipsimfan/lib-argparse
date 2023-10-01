use crate::{FlagArgument, Parser, TerminalArgument};
use header::__parser_header;
use std::borrow::Cow;

mod flag;
mod header;
mod terminal;

#[doc(hidden)]
pub use flag::{__flag, __name_flag};
#[doc(hidden)]
pub use terminal::__command;

#[macro_export]
/// Creates a [`Parser`]
///
///  - `program_name` is the program name
///  - `description` is the description
///  - `terminal` is the terminal argument
macro_rules! parser {
    {} => {
        $crate::macros::__parser(
            ::std::option::Option::None,
            ::std::option::Option::None,
            $crate::TerminalArgument::None,
            ::std::vec::Vec::new(),
        )
    };

    {
        $program_name: literal
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::None,
            $crate::TerminalArgument::None,
            ::std::vec::Vec::new(),
        )
    };


    {
        $program_name: literal
        $description: literal
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::Some($description.into()),
            $crate::TerminalArgument::None,
            ::std::vec::Vec::new(),
        )
    };

    {
        terminal: expr
    } => {
        $crate::macros::__parser(
            ::std::option::Option::None,
            ::std::option::Option::None,
            $terminal,
            ::std::vec::Vec::new(),
        )
    };

    {
        $program_name: literal
        $terminal: expr
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::None,
            $terminal,
            ::std::vec::Vec::new(),
        )
    };


    {
        $program_name: literal
        $description: literal
        $terminal: expr
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::Some($description.into()),
            $terminal,
            ::std::vec::Vec::new(),
        )
    };

    {
        None,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::None,
            ::std::option::Option::None,
            $crate::TerminalArgument::None,
            ::std::vec![$($flag),*],
        )
    };

    {
        $terminal: expr,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::None,
            ::std::option::Option::None,
            $terminal,
            ::std::vec![$($flag),*],
        )
    };

    {
        $program_name: literal
        None,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::None,
            $crate::TerminalArgument::None,
            ::std::vec![$($flag),*],
        )
    };

    {
        $program_name: literal
        $terminal: expr,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::None,
            $terminal,
            ::std::vec![$($flag),*],
        )
    };

    {
        $program_name: literal
        $description: literal
        None,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::Some($description.into()),
            $crate::TerminalArgument::None,
            ::std::vec![$($flag),*],
        )
    };

    {
        $program_name: literal
        $description: literal
        $terminal: expr,
        [$($flag: expr),*]
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::Some($description.into()),
            $terminal,
            ::std::vec![$($flag),*],
        )
    };
}

#[doc(hidden)]
/// Creates a [`Parser`]
///
///  - `program_name` is the program name
///  - `description` is the description
///  - `terminal` is the terminal argument
///  - `flags` is the list of flag arguments
pub fn __parser<T, E>(
    program_name: Option<Cow<'static, str>>,
    description: Option<Cow<'static, str>>,
    terminal: TerminalArgument<T, E>,
    flags: Vec<FlagArgument<T, E>>,
) -> Parser<T, E> {
    let mut parser = __parser_header(program_name, description).set_terminal_argument(terminal);

    for flag in flags {
        parser.add_flag_argument(flag);
    }

    parser
}
