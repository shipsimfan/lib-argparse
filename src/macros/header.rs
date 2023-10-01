use std::borrow::Cow;

use crate::Parser;

#[doc(hidden)]
/// Creates a [`Parser`] with only defined header elements
///
///  - `program_name` is the program name
///  - `description` is the description
pub(super) fn __parser_header<T, E>(
    program_name: Option<Cow<'static, str>>,
    description: Option<Cow<'static, str>>,
) -> Parser<T, E> {
    let mut parser = crate::Parser::new();

    parser = match program_name {
        Some(program_name) => parser.set_program_name(program_name),
        None => parser,
    };

    match description {
        Some(description) => parser.set_description(description),
        None => parser,
    }
}
