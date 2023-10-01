use crate::{Parser, TerminalArgument};
use header::__parser_header;
use std::borrow::Cow;

mod command;
mod header;

#[doc(hidden)]
pub use command::__command;

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
        )
    };

    {
        $program_name: literal
    } => {
        $crate::macros::__parser(
            ::std::option::Option::Some($program_name.into()),
            ::std::option::Option::None,
            $crate::TerminalArgument::None,
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
        )
    };

    {
        terminal: expr
    } => {
        $crate::macros::__parser(
            ::std::option::Option::None,
            ::std::option::Option::None,
            $terminal,
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
        )
    };
}

#[doc(hidden)]
/// Creates a [`Parser`]
///
///  - `program_name` is the program name
///  - `description` is the description
///  - `terminal` is the terminal argument
pub fn __parser<T, E>(
    program_name: Option<Cow<'static, str>>,
    description: Option<Cow<'static, str>>,
    terminal: TerminalArgument<T, E>,
) -> Parser<T, E> {
    __parser_header(program_name, description).set_terminal_argument(terminal)
}
