use crate::FlagArgument;

/// Generates the flag usage
pub(super) fn generate<'a, Options>(
    header: Option<&dyn std::fmt::Display>,
    flags: &'a [&'a dyn FlagArgument<'a, Options>],
    short_prefix: &str,
    long_prefix: &str,
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
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
            output,
        )?;
    }

    Ok(())
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
    output: &mut dyn std::fmt::Write,
) -> std::fmt::Result {
    if flags.len() == 0 {
        return Ok(());
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
    writeln!(output,)?;
    writeln!(output, "{}:", header)?;
    for flag in flags {
        let mut total_printed = 0;

        write!(output, "  ")?;
        total_printed += 2;

        if let Some(short_name) = flag.short_name() {
            write!(output, "{}{}", short_prefix, short_name)?;
            total_printed += short_prefix_length + short_name.len();

            if flag.long_name().is_some() {
                write!(output, ", ")?;
                total_printed += 2;
            }
        }

        if let Some(long_name) = flag.long_name() {
            write!(output, "{}{}", long_prefix, long_name)?;
            total_printed += long_prefix_length + long_name.len();
        }

        if let Some(hint) = flag.hint() {
            write!(output, " {}", hint)?;
            total_printed += 1 + hint.to_string().len();
        }

        for _ in total_printed..description_offset {
            write!(output, " ")?;
        }

        let mut first = true;
        for description_line in flag.description().unwrap_or(&[]) {
            if first {
                first = false;
            } else {
                for _ in 0..description_offset {
                    write!(output, " ")?;
                }
            }

            writeln!(output, "{}", description_line)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{FlagArgument, HelpFlagArgument};

    use super::partition_flags;

    #[test]
    fn partition_flags_empty() {
        assert_eq!(partition_flags::<()>(&[]).len(), 0);
    }

    #[test]
    fn partition_one_group_one_entry() {
        let flags: [&dyn FlagArgument<()>; 1] = [&HelpFlagArgument::new().short_name("a")];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 1);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 1);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"))
    }

    #[test]
    fn partition_one_group_two_entries() {
        let flags: [&dyn FlagArgument<()>; 2] = [
            &HelpFlagArgument::new().short_name("a"),
            &HelpFlagArgument::new().short_name("b"),
        ];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 1);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 2);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"));
        assert_eq!(partitions[0].1[1].short_name(), Some("b"));
    }

    #[test]
    fn partition_two_groups_one_entry() {
        let flags: [&dyn FlagArgument<()>; 2] = [
            &HelpFlagArgument::new().short_name("a"),
            &HelpFlagArgument::new().short_name("b").group("A"),
        ];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 2);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 1);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"));

        assert_eq!(partitions[1].0, Some("A"));
        assert_eq!(partitions[1].1.len(), 1);
        assert_eq!(partitions[1].1[0].short_name(), Some("b"));
    }

    #[test]
    fn partition_two_groups_two_entries() {
        let flags: [&dyn FlagArgument<()>; 4] = [
            &HelpFlagArgument::new().short_name("a"),
            &HelpFlagArgument::new().short_name("b").group("A"),
            &HelpFlagArgument::new().short_name("c"),
            &HelpFlagArgument::new().short_name("d").group("A"),
        ];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 2);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 2);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"));
        assert_eq!(partitions[0].1[1].short_name(), Some("c"));

        assert_eq!(partitions[1].0, Some("A"));
        assert_eq!(partitions[1].1.len(), 2);
        assert_eq!(partitions[1].1[0].short_name(), Some("b"));
        assert_eq!(partitions[1].1[1].short_name(), Some("d"));
    }

    #[test]
    fn partition_three_groups_one_entry() {
        let flags: [&dyn FlagArgument<()>; 3] = [
            &HelpFlagArgument::new().short_name("a"),
            &HelpFlagArgument::new().short_name("b").group("A"),
            &HelpFlagArgument::new().short_name("c").group("B"),
        ];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 3);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 1);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"));

        assert_eq!(partitions[1].0, Some("A"));
        assert_eq!(partitions[1].1.len(), 1);
        assert_eq!(partitions[1].1[0].short_name(), Some("b"));

        assert_eq!(partitions[2].0, Some("B"));
        assert_eq!(partitions[2].1.len(), 1);
        assert_eq!(partitions[2].1[0].short_name(), Some("c"));
    }

    #[test]
    fn partition_three_groups_two_entries() {
        let flags: [&dyn FlagArgument<()>; 6] = [
            &HelpFlagArgument::new().short_name("a"),
            &HelpFlagArgument::new().short_name("b").group("A"),
            &HelpFlagArgument::new().short_name("c").group("B"),
            &HelpFlagArgument::new().short_name("d"),
            &HelpFlagArgument::new().short_name("e").group("A"),
            &HelpFlagArgument::new().short_name("f").group("B"),
        ];

        let partitions = partition_flags::<()>(&flags);

        assert_eq!(partitions.len(), 3);

        assert_eq!(partitions[0].0, None);
        assert_eq!(partitions[0].1.len(), 2);
        assert_eq!(partitions[0].1[0].short_name(), Some("a"));
        assert_eq!(partitions[0].1[1].short_name(), Some("d"));

        assert_eq!(partitions[1].0, Some("A"));
        assert_eq!(partitions[1].1.len(), 2);
        assert_eq!(partitions[1].1[0].short_name(), Some("b"));
        assert_eq!(partitions[1].1[1].short_name(), Some("e"));

        assert_eq!(partitions[2].0, Some("B"));
        assert_eq!(partitions[2].1.len(), 2);
        assert_eq!(partitions[2].1[0].short_name(), Some("c"));
        assert_eq!(partitions[2].1[1].short_name(), Some("f"));
    }
}
