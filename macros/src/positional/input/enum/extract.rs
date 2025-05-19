use super::{EnumInput, EnumInputVariant};
use proc_macro_util::{ast::items::Enumeration, Error};

impl<'a> EnumInput<'a> {
    /// Extract the required details from `r#enum`
    pub fn extract(r#enum: Enumeration<'a>) -> Result<Self, Error> {
        let mut variants = Vec::new();
        if let Some(items) = r#enum.enum_items {
            variants.push(EnumInputVariant::extract(items.first)?);
            for (_, item) in items.remaining {
                variants.push(EnumInputVariant::extract(item)?);
            }
        }

        Ok(EnumInput {
            name: r#enum.name,
            variants,
        })
    }
}
