use super::EnumInputVariant;
use proc_macro_util::{
    ast::items::{EnumItem, EnumItemKind},
    Result,
};

impl<'a> EnumInputVariant<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: EnumItem<'a>) -> Result<Self> {
        let r#type = match item.kind {
            Some(EnumItemKind::Tuple(Some(tuple))) => {
                if tuple.remaining.len() > 0 {
                    return Err(item.name.span().error(
                        "`Positional` derive only supports tuple-style enum variants with one field",
                    ));
                } else {
                    Some(tuple.first.r#type)
                }
            }
            Some(EnumItemKind::Tuple(None)) => None,
            Some(_) => {
                return Err(item
                    .name
                    .span()
                    .error("`Positional` derive does not support struct-style enum variants"))
            }
            None => None,
        };

        Ok(EnumInputVariant {
            name: item.name,
            r#type,
        })
    }
}
