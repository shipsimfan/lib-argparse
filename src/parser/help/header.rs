/// Generates the help header
///
/// Returns `true` if this printed a value
pub(super) fn generate(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
    output: &mut dyn std::fmt::Write,
) -> Result<bool, std::fmt::Error> {
    let mut printed = false;

    if let Some(name) = name {
        writeln!(output, "{}", name)?;
        printed = true;
    }

    if let Some(description) = description {
        writeln!(output, "{}", description)?;
        printed = true;
    }

    Ok(printed)
}

#[cfg(test)]
mod tests {
    use super::generate;

    #[test]
    fn header_empty() {
        let mut output = String::new();
        generate(None, None, &mut output).unwrap();
        assert_eq!(output, "");
    }

    #[test]
    fn header_name() {
        let mut output = String::new();
        generate(Some(&"Name"), None, &mut output).unwrap();
        assert_eq!(output, "Name\n");
    }

    #[test]
    fn header_description() {
        let mut output = String::new();
        generate(None, Some(&"A description"), &mut output).unwrap();
        assert_eq!(output, "A description\n");
    }

    #[test]
    fn header_both() {
        let mut output = String::new();
        generate(Some(&"Name"), Some(&"A description"), &mut output).unwrap();
        assert_eq!(output, "Name\nA description\n");
    }
}
