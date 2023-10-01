#[macro_export]
#[doc(hidden)]
/// Creates a `Parser` with only defined header elements
///
///  - `program_name` is an `Option<Into<Cow<'static, str>>>` with the program name
///  - `description` is an `Option<Into<Cow<'static, str>>>` with the description
macro_rules! __parser_header_inner {
    ($program_name: expr, $description: expr) => {{
        let mut parser = $crate::Parser::new();
        parser = match $program_name {
            ::std::option::Option::Some(program_name) => parser.set_program_name(program_name),
            ::std::option::Option::None => parser,
        };
        match $description {
            ::std::option::Option::Some(description) => description.set_description(description),
            ::std::option::Option::None => parser,
        }
    }};
}
