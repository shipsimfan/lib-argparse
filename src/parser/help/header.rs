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
    #[test]
    fn header_empty() {}
}
