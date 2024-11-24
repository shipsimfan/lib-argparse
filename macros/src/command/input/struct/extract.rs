use super::StructInput;
use proc_macro_util::ast::items::Struct;

impl StructInput {
    /// Extract the required details from `r#struct`
    pub fn extract(r#struct: Struct) -> Self {
        let name = r#struct.name.into_owned();

        StructInput { name }
    }
}
