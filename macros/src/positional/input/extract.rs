use super::{EnumInput, Input};
use proc_macro_util::{
    ast::{DeriveItem, DeriveItemKind},
    Error,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self, Error> {
        match item.kind {
            DeriveItemKind::Enum(r#enum) => Ok(Input::Enum(EnumInput::extract(r#enum)?)),
            _ => Err(Error::new("`FlagGroup` derive only supports structs")),
        }
    }
}
