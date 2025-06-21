use super::{Input, StructInput};
use proc_macro_util::{
    ast::{DeriveItem, DeriveItemKind},
    Result, Span,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self> {
        match item.kind {
            DeriveItemKind::Struct(r#struct) => Ok(Input::Struct(StructInput::extract(r#struct)?)),
            _ => Err(Span::call_site().error("`FlagGroup` derive only supports structs")),
        }
    }
}
