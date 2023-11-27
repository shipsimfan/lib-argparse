/// Generates the help header
///
/// Returns `true` if this printed a value
pub(super) fn generate(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
) -> bool {
    let mut printed = false;

    if let Some(name) = name {
        println!("{}", name);
        printed = true;
    }

    if let Some(description) = description {
        println!("{}", description);
        printed = true;
    }

    printed
}
