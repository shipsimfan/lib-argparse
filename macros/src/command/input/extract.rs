use super::{Input, StructInput};
use proc_macro_util::ast::{DeriveItem, DeriveItemKind};

impl Input {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem) -> Self {
        match item.kind {
            DeriveItemKind::Struct(r#struct) => Input::Struct(StructInput::extract(r#struct)),
        }
    }
}
