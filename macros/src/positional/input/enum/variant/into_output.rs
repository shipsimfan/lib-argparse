use super::EnumInputVariant;
use crate::positional::output::{EnumVariantDisplay, EnumVariantParse, EnumVariantSub};
use proc_macro_util::tokens::Literal;

impl<'a> EnumInputVariant<'a> {
    /// Converts this input into the needed output types
    pub fn into_output(
        self,
    ) -> (
        EnumVariantParse<'a>,
        String,
        Option<EnumVariantSub<'a>>,
        EnumVariantDisplay<'a>,
    ) {
        let mut string = self.name.to_string();
        let mut i = 0;
        let mut last_lowercase = false;
        while i < string.len() {
            if string.as_bytes()[i].is_ascii_lowercase() {
                last_lowercase = true;
            } else if last_lowercase && string.as_bytes()[i].is_ascii_uppercase() {
                string.insert(i, '-');
                last_lowercase = false;
                i += 1;
            }

            i += 1;
        }

        let string = string.to_lowercase();
        let string_literal = Literal::new(string.as_str());
        let has_field = self.r#type.is_some();

        (
            EnumVariantParse::new(
                string_literal.clone(),
                if has_field {
                    None
                } else {
                    Some(self.name.clone())
                },
            ),
            string,
            self.r#type.map(|r#type| {
                EnumVariantSub::new(string_literal.clone(), r#type, self.name.clone())
            }),
            EnumVariantDisplay::new(self.name, has_field, string_literal),
        )
    }
}
