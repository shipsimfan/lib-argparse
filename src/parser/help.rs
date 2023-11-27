pub(super) fn generate(
    name: Option<&dyn std::fmt::Display>,
    description: Option<&dyn std::fmt::Display>,
) {
    generate_header(name, description);
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
