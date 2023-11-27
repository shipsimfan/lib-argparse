use std::{ffi::OsString, iter::Peekable, str::Chars};

use crate::FlagArgument;

pub(super) fn generate<Options>(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
    usage: Option<&str>,
    command_list: &[OsString],
    prologue: Option<&dyn std::fmt::Display>,
    header: Option<&dyn std::fmt::Display>,
    flags: &[&dyn FlagArgument<Options>],
    epilogue: Option<&dyn std::fmt::Display>,
) {
    generate_header(name, description);
    generate_usage(usage, command_list);
    println!();

    if let Some(prologue) = prologue {
        println!("{}", prologue);
        println!();
    }

    generate_flags(header, flags);

    if let Some(epilogue) = epilogue {
        println!();
        println!("{}", epilogue);
    }
}

fn generate_header(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
) {
    let mut printed = false;

    if let Some(name) = name {
        println!("{}", name);
        printed = true;
    }

    if let Some(description) = description {
        println!("{}", description);
        printed = true;
    }

    if printed {
        println!();
    }
}

fn generate_usage(usage: Option<&str>, command_list: &[OsString]) {
    if let Some(usage) = usage {
        parse_usage(usage.chars().peekable(), command_list);
    } else {
        println!("USAGE:");
        print!("    ");
        display_command_list(command_list);
        print!(" [OPTIONS] ");
        // TODO: display_terminal_usage
        println!();
    }
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
                't' => todo!("terminal usage"),
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

fn generate_flags<Options>(
    header: Option<&dyn std::fmt::Display>,
    flags: &[&dyn FlagArgument<Options>],
) {
    todo!()
}
