use super::Positional;
use proc_macro_util::{
    ast::items::StructField,
    tokens::{Identifier, Literal},
};

impl<'a> Positional<'a> {
    pub fn extract(field: StructField<'a>) -> Self {
        let variable_name = field.name.into_owned();
        let r#type = field.r#type;

        let name_upper = variable_name.to_string().to_uppercase();
        let info_name = Identifier::new(&format!("__{}_INFO", name_upper));
        let value = Literal::new(name_upper.replace('_', "-").as_str());

        Positional {
            variable_name,
            info_name,
            r#type,
            value,
            min_count: Literal::new(0),
            max_count: Literal::new(0),
            default: None,
        }
    }
}
