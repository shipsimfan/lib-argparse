use super::{EnumInput, Input};
use proc_macro_util::{
    ast::{DeriveItem, DeriveItemKind},
    Result, Span,
};

impl<'a> Input<'a> {
    /// Extract the required details from `item`
    pub fn extract(item: DeriveItem<'a>) -> Result<Self> {
        match item.kind {
            DeriveItemKind::Enum(r#enum) => Ok(Input::Enum(EnumInput::extract(r#enum)?)),
            _ => Err(Span::call_site().error("`FlagGroup` derive only supports structs")),
        }
    }
}
