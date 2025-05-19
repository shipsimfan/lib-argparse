use super::EnumInputVariant;
use proc_macro_util::{
    ast::items::{EnumItem, EnumItemKind},
    Error,
};

impl<'a> EnumInputVariant<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: EnumItem<'a>) -> Result<Self, Error> {
        let r#type = match item.kind {
            Some(EnumItemKind::Tuple(Some(tuple))) => {
                if tuple.remaining.len() > 0 {
                    return Err(Error::new_at(
                        "`Positional` derive only supports tuple-style enum variants with one field",
                        item.name.span(),
                    ));
                } else {
                    Some(tuple.first.r#type)
                }
            }
            Some(EnumItemKind::Tuple(None)) => None,
            Some(_) => {
                return Err(Error::new_at(
                    "`Positional` derive does not support struct-style enum variants",
                    item.name.span(),
                ))
            }
            None => None,
        };

        Ok(EnumInputVariant {
            name: item.name,
            r#type,
        })
    }
}
