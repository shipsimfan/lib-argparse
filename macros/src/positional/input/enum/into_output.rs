use super::EnumInput;
use crate::positional::output::{EnumOutput, Output};
use proc_macro_util::tokens::Literal;

impl<'a> EnumInput<'a> {
    /// Converts this input into an [`Output`]
    pub fn into_output(self) -> Output<'a> {
        let mut parses = Vec::with_capacity(self.variants.len());
        let mut subs = Vec::with_capacity(self.variants.len());
        let mut displays = Vec::with_capacity(self.variants.len());
        let mut expected = String::new();
        let last = self.variants.len().checked_sub(1).unwrap_or(0);
        for (i, variant) in self.variants.into_iter().enumerate() {
            let (parse, name, sub, display) = variant.into_output();

            parses.push(parse);
            displays.push(display);

            if let Some(sub) = sub {
                subs.push(sub);
            }

            if i > 0 {
                expected.push_str(", ");
            }

            if i != 0 && i == last {
                expected.push_str("or ");
            }
            expected.push('"');
            expected.push_str(&name);
            expected.push('"');
        }

        Output::Enum(EnumOutput::new(
            self.name,
            parses,
            Literal::new(expected.as_str()),
            subs,
            displays,
        ))
    }
}
