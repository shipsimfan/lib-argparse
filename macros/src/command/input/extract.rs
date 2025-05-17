use super::{CommandInfo, Input, StructInput};
use proc_macro_util::{
    ast::{AttrInput, DeriveItem, DeriveItemKind},
    Error,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self, Error> {
        let mut command_attribute = None;
        let mut docs = Vec::new();
        for attribute in item.attributes {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            match attribute.attr.path.first.to_string().as_str() {
                "command" => command_attribute = Some(attribute),
                "doc" => match attribute.attr.input {
                    Some(AttrInput::Expression(_, expression)) => docs.push(expression),
                    _ => {}
                },
                _ => {}
            }
        }

        let info = if let Some(attribute) = command_attribute {
            CommandInfo::extract(attribute, docs)?
        } else {
            CommandInfo::default()
        };

        match item.kind {
            DeriveItemKind::Struct(r#struct) => {
                Ok(Input::Struct(StructInput::extract(r#struct, info)?))
            }
            DeriveItemKind::Enum(r#enum) => todo!(),
        }
    }
}
