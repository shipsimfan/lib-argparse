#[macro_export]
/// Creates a [`Positionals`]
///
///  - `help_description` is the description of a positional in the help
///  - `positional` is the parser for a positional
macro_rules! positionals {
    [$($help_description: literal $positional: expr),*] => {{
        let mut positionals = $crate::Positionals::new();

        $(positionals.push($positional, $help_description);)*

        $crate::TerminalArgument::Positionals(positionals)
    }};
}
