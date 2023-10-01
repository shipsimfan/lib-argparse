use crate::{Parser, TerminalArgument};
use header::__parser_header;
use std::borrow::Cow;

mod header;

#[macro_export]
/// Creates a [`Parser`]
///
///  - `program_name` is the program name
///  - `description` is the description
///  - `terminal` is the terminal argument
macro_rules! parser {
    {} => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::None,
            ::std::option::Option::None,
        }
    };

    {
        $program_name: literal
    } => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::None,
        }
    };


    {
        $program_name: literal
        $description: literal
    } => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::Some($description),
        }
    };

    {
        terminal: expr
    } => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::None,
            ::std::option::Option::None,
        }
    };

    {
        $program_name: literal
        $terminal: expr
    } => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::None,
        }
    };


    {
        $program_name: literal
        $description: literal
        $terminal: expr
    } => {
        $crate::macros::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::Some($description),
        }
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
