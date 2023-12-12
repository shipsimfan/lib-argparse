use std::{ffi::OsString, iter::Peekable, str::Chars};

const DEFAULT_USAGE: &str = "USAGE:\n    %c %t";

/// Generates the usage based on the provided usage string
pub(super) fn generate(
    usage: Option<&str>,
    command_list: &[OsString],
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    parse_usage(
        usage.unwrap_or(DEFAULT_USAGE).chars().peekable(),
        command_list,
        output,
    )
}

fn parse_usage(
    mut usage: Peekable<Chars>,
    command_list: &[OsString],
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    while let Some(c) = usage.next() {
        if c == '\\' {
            if let Some(c) = usage.next() {
                write!(output, "{}", c)?;
            } else {
                write!(output, "\\")?;
            }
        } else if c == '%' {
            let c = match usage.next() {
                Some(c) => c,
                None => return write!(output, "%"),
            };

            match c {
                't' => Ok(()), // TODO: Terminal Usage,
                'c' => display_command_list(command_list, output),
                x if x.is_digit(10) => {
                    let mut value = x.to_digit(10).unwrap() as usize;
                    while let Some(c) = usage.peek() {
                        if let Some(digit) = c.to_digit(10) {
                            value *= 10;
                            value += digit as usize;
                            usage.next();
                        } else {
                            break;
                        }
                    }

                    write!(output, "{}", command_list[value].to_string_lossy())
                }
                _ => write!(output, "%{}", c),
            }?
        } else {
            write!(output, "{}", c)?;
        }
    }

    writeln!(output)
}

fn display_command_list(
    command_list: &[OsString],
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    let mut first = true;
    for command in command_list {
        if first {
            first = false;
        } else {
            write!(output, " ")?;
        }
        write!(output, "{}", command.to_string_lossy())?;
    }
    Ok(())
}
