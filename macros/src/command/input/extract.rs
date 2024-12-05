use super::{Input, StructInput};
use proc_macro_util::{
    ast::{DeriveItem, DeriveItemKind},
    Error,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self, Error> {
        let mut command_attribute = None;
        for attribute in item.attributes {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            if attribute.attr.path.first.to_string().as_str() == "command" {
                command_attribute = Some(attribute);
                break;
            }
        }

        match item.kind {
            DeriveItemKind::Struct(r#struct) => Ok(Input::Struct(StructInput::extract(r#struct)?)),
        }
    }
}
