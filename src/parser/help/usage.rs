use std::{ffi::OsString, iter::Peekable, str::Chars};

const DEFAULT_USAGE: &str = "USAGE:\n    %c %t";

/// Generates the usage based on the provided usage string
pub(super) fn generate(usage: Option<&str>, command_list: &[OsString]) {
    parse_usage(
        usage.unwrap_or(DEFAULT_USAGE).chars().peekable(),
        command_list,
    );
}

fn parse_usage(mut usage: Peekable<Chars>, command_list: &[OsString]) {
    while let Some(c) = usage.next() {
        if c == '\\' {
            if let Some(c) = usage.next() {
                print!("{}", c);
            } else {
                print!("\\");
            }
        } else if c == '%' {
            let c = match usage.next() {
                Some(c) => c,
                None => return print!("%"),
            };

            match c {
                't' => {} // TODO: Terminal Usage,
                'c' => display_command_list(command_list),
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
                    print!("{}", command_list[value].to_string_lossy());
                }
                _ => print!("%{}", c),
            }
        } else {
            print!("{}", c);
        }
    }
    println!();
}

fn display_command_list(command_list: &[OsString]) {
    let mut first = true;
    for command in command_list {
        if first {
            first = false;
        } else {
            print!(" ");
        }
        print!("{}", command.to_string_lossy());
    }
}
