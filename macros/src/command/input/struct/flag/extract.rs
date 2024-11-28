use super::Flag;
use proc_macro_util::{
    ast::items::StructField,
    tokens::{Identifier, Literal},
};

impl<'a> Flag<'a> {
    /// Tries to extract a [`Flag`] from `field`, return the field in [`Err`] if the field is not a
    /// flag
    pub fn extract(mut field: StructField<'a>) -> Result<Self, StructField<'a>> {
        let mut flag_attribute = None;
        for (i, attribute) in field.attributes.iter().enumerate() {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            if attribute.attr.path.first.to_string().as_str() == "flag" {
                flag_attribute = Some(i);
                break;
            }
        }

        let flag_attribute = match flag_attribute {
            Some(index) => field.attributes.swap_remove(index),
            None => return Err(field),
        };

        let variable_name = field.name.into_owned();
        let r#type = field.r#type;

        let variable_name_str = variable_name.to_string();
        let long_name_str = variable_name_str.replace('_', "-");
        let long_name = Literal::new(long_name_str.as_str());
        let info_long_name = Literal::new(format!("--{}", long_name_str).as_str());

        let name_upper = variable_name_str.to_uppercase();
        let info_name = Identifier::new(&format!("__{}_INFO", name_upper));

        Ok(Flag {
            variable_name,
            info_name,
            r#type,
            long_name,
            info_long_name,
            short_name: None,
            info_short_name: None,
            value: None,
            min_count: Literal::new(0),
            max_count: Literal::new(0),
            default: None,
        })
    }
}
