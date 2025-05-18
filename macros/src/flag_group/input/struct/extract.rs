use super::{Flag, StructInput};
use proc_macro_util::{
    ast::items::{Struct, StructBody},
    Error,
};

impl<'a> StructInput<'a> {
    /// Extract the required details from `r#struct`
    pub fn extract(r#struct: Struct<'a>) -> Result<Self, Error> {
        let fields = match r#struct.body {
            StructBody::Normal {
                where_clause: _,
                fields,
            } => fields,
            _ => {
                return Err(Error::new_at(
                    "`FlagGroup` only supports structs with named fields",
                    r#struct.name.span(),
                ))
            }
        };

        let mut flags = Vec::new();
        if let Some(fields) = fields {
            flags.push(Flag::extract(fields.first)?);

            for (_, field) in fields.remaining {
                flags.push(Flag::extract(field)?);
            }
        }

        Ok(StructInput {
            name: r#struct.name,
            flags,
        })
    }
}
