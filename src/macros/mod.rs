mod header;

#[macro_export]
/// Creates a `Parser`
///
///  - `program_name` is an `Option<Into<Cow<'static, str>>>` with the program name
///  - `description` is an `Option<Into<Cow<'static, str>>>` with the description
macro_rules! parser {
    {} => {
        $crate::__parser_inner! {
            ::std::option::Option::None,
            ::std::option::Option::None,
        }
    };

    {
        $program_name: literal
    } => {
        $crate::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::None,
        }
    };


    {
        $program_name: literal
        $description: literal
    } => {
        $crate::__parser_inner! {
            ::std::option::Option::Some($program_name),
            ::std::option::Option::Some($description),
        }
    };
}

#[macro_export]
#[doc(hidden)]
/// Creates a `Parser`
///
///  - `program_name` is an `Option<Into<Cow<'static, str>>>` with the program name
///  - `description` is an `Option<Into<Cow<'static, str>>>` with the description
macro_rules! __parser_inner {
    {
        $program_name: expr,
        $description: expr,
    } => {
        $crate::__parser_header_inner!($program_name, $description)
    };
}
