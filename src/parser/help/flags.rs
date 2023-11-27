use crate::FlagArgument;

/// Generates the flag usage
pub(super) fn generate<'a, Options>(
    header: Option<&dyn std::fmt::Display>,
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
    short_prefix: &str,
    long_prefix: &str,
) {
    let groups = partition_flags(flags);

    for (group_header, group) in groups {
        display_group(
            match &group_header {
                Some(group_header) => group_header,
                None => header.unwrap_or(&"FLAGS"),
            },
            group,
            short_prefix,
            long_prefix,
        );
    }
}

fn partition_flags<'a, Options>(
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
) -> Vec<(Option<&'a str>, Vec<&'a dyn FlagArgument<'a, Options>>)> {
    let mut partitions: Vec<(Option<&str>, Vec<&dyn FlagArgument<Options>>)> = Vec::new();

    'main: for flag in flags {
        for (group_name, group) in &mut partitions {
            if *group_name == flag.group() {
                group.push(*flag);
                continue 'main;
            }
        }

        partitions.push((flag.group(), vec![*flag]));
    }

    partitions
}

fn display_group<'a, Options: 'a>(
    header: &dyn std::fmt::Display,
    flags: Vec<&'a dyn FlagArgument<'a, Options>>,
    short_prefix: &str,
    long_prefix: &str,
) {
    if flags.len() == 0 {
        return;
    }

    // Calculate description offset
    let short_prefix_length = short_prefix.len();
    let long_prefix_length = long_prefix.len();

    let mut longest_length = 0;
    for flag in &flags {
        let short_length = flag
            .short_name()
            .map(|short_name| short_name.len() + short_prefix_length)
            .unwrap_or(0);

        let long_length = flag
            .long_name()
            .map(|long_name| long_name.len() + long_prefix_length)
            .unwrap_or(0);

        let hint_length = flag
            .hint()
            .map(|hint| hint.to_string().len() + 1)
            .unwrap_or(0);

        let length = short_length
            + long_length
            + hint_length
            + if flag.long_name().is_some() && flag.short_name().is_some() {
                2
            } else {
                0
            };

        if length > longest_length {
            longest_length = length;
        }
    }

    let description_offset = longest_length + 4;

    // Display flags
    println!();
    println!("{}:", header);
    for flag in flags {
        let mut total_printed = 0;

        print!("  ");
        total_printed += 2;

        if let Some(short_name) = flag.short_name() {
            print!("{}{}", short_prefix, short_name);
            total_printed += short_prefix_length + short_name.len();

            if flag.long_name().is_some() {
                print!(", ");
                total_printed += 2;
            }
        }

        if let Some(long_name) = flag.long_name() {
            print!("{}{}", long_prefix, long_name);
            total_printed += long_prefix_length + long_name.len();
        }

        if let Some(hint) = flag.hint() {
            print!(" {}", hint);
            total_printed += 1 + hint.to_string().len();
        }

        for _ in total_printed..description_offset {
            print!(" ");
        }

        let mut first = true;
        for description_line in flag.description().unwrap_or(&[]) {
            if first {
                first = false;
            } else {
                for _ in 0..description_offset {
                    print!(" ");
                }
            }

            println!("{}", description_line);
        }
    }
}
