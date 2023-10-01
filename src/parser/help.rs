use crate::Parser;

pub(super) struct HelpGenerator<'a, T, E: 'static> {
    parser: &'a Parser<T, E>,
    command_chain: &'a [String],
    first_argument: &'a str,
    program_name: Option<&'a str>,
    description: Option<&'a str>,
}

/// Generates a the help header
///
///  - `f` is the output
///  - `parser` is the parser to generate for
///
/// OUTPUT:
/// > Program Name\
/// > Program Description
fn generate_header(
    f: &mut std::fmt::Formatter<'_>,
    program_name: Option<&str>,
    description: Option<&str>,
) -> std::fmt::Result {
    let mut header = false;
    if let Some(program_name) = program_name {
        writeln!(f, "{}", program_name)?;
        header = true;
    }

    if let Some(description) = description {
        writeln!(f, "{}", description)?;
        header = true;
    }

    if header {
        writeln!(f)?;
    }

    Ok(())
}

/// Generates a the usage
///
///  - `f` is the output
///  - `parser` is the parser to generate for
///  - `command_chain` is the list of command before `parser`
///
/// OUTPUT:
/// > USAGE:\
/// >     program.exe command chain \[OPTIONS..\] terminal
fn generate_usage<T, E>(
    f: &mut std::fmt::Formatter<'_>,
    parser: &Parser<T, E>,
    command_chain: &[String],
    first_argument: &str,
) -> std::fmt::Result {
    writeln!(f, "USAGE:")?;

    write!(f, "    {}", first_argument)?;
    for command in command_chain {
        write!(f, " {}", command)?;
    }

    if parser.flag_arguments.len() > 0 {
        write!(f, " [OPTIONS..]")?;
    }

    parser.terminal_argument.generate_usage(f)?;

    writeln!(f)
}

impl<'a, T, E> HelpGenerator<'a, T, E> {
    /// Creates a new `HelpGenerator`
    ///
    ///  - `parser` is the parser to generate for
    ///  - `command_chain` is the list of commands preceding `parser`
    ///  - `first_argument` is the first argument provided by the operating system
    pub(super) fn new(
        parser: &'a Parser<T, E>,
        command_chain: &'a [String],
        first_argument: &'a str,
        program_name: Option<&'a str>,
        description: Option<&'a str>,
    ) -> Self {
        HelpGenerator {
            parser,
            command_chain,
            first_argument,
            program_name,
            description,
        }
    }
}

impl<'a, T, E> std::fmt::Display for HelpGenerator<'a, T, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        generate_header(f, self.program_name, self.description)?;

        generate_usage(f, self.parser, self.command_chain, self.first_argument)?;

        self.parser.terminal_argument.generate_help(f)?;

        self.parser.flag_arguments.generate_help(
            f,
            &self.parser.short_prefix,
            &self.parser.long_prefix,
        )
    }
}
