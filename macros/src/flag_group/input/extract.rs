use super::{Input, StructInput};
use proc_macro_util::{
    ast::{DeriveItem, DeriveItemKind},
    Error,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self, Error> {
        match item.kind {
            DeriveItemKind::Struct(r#struct) => Ok(Input::Struct(StructInput::extract(r#struct)?)),
            _ => Err(Error::new("`FlagGroup` derive only supports structs")),
        }
    }
}
