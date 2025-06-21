use super::{Flag, FlagGroup, StructInput};
use proc_macro_util::{
    ast::items::{Struct, StructBody},
    Result,
};

impl<'a> StructInput<'a> {
    /// Extract the required details from `r#struct`
    pub fn extract(r#struct: Struct<'a>) -> Result<Self> {
        let fields = match r#struct.body {
            StructBody::Normal {
                where_clause: _,
                fields,
            } => fields,
            _ => {
                return Err(r#struct
                    .name
                    .span()
                    .error("`FlagGroup` only supports structs with named fields"))
            }
        };

        let mut flags = Vec::new();
        let mut flag_groups = Vec::new();
        if let Some(fields) = fields {
            match FlagGroup::extract(fields.first)? {
                Ok(flag_group) => flag_groups.push(flag_group),
                Err(field) => flags.push(Flag::extract(field)?),
            }

            for (_, field) in fields.remaining {
                match FlagGroup::extract(field)? {
                    Ok(flag_group) => flag_groups.push(flag_group),
                    Err(field) => flags.push(Flag::extract(field)?),
                }
            }
        }

        Ok(StructInput {
            name: r#struct.name,
            flags,
            flag_groups,
        })
    }
}
