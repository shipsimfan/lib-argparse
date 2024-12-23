use super::{Flag, Positional, StructInput};
use crate::command::input::CommandInfo;
use proc_macro_util::{
    ast::items::{Struct, StructBody},
    Error,
};

impl<'a> StructInput<'a> {
    /// Extract the required details from `r#struct`
    pub fn extract(r#struct: Struct<'a>, info: CommandInfo<'a>) -> Result<Self, Error> {
        let name = r#struct.name.into_owned();

        let fields = match r#struct.body {
            StructBody::Normal {
                where_clause: _,
                fields,
            } => match fields {
                Some(raw_fields) => {
                    let mut fields = Vec::with_capacity(raw_fields.remaining.len() + 1);
                    fields.push(raw_fields.first);
                    for (_, field) in raw_fields.remaining {
                        fields.push(field);
                    }
                    fields
                }
                None => Vec::new(),
            },
            StructBody::Empty {
                where_clause: _,
                semi: _,
            } => Vec::new(),
            StructBody::Tuple {
                fields: _,
                where_clause: _,
                semi: _,
            } => todo!("Tuple structs for Command"),
        };

        let mut positionals = Vec::new();
        let mut flags = Vec::new();
        for field in fields {
            match Flag::extract(field)? {
                Ok(flag) => flags.push(flag),
                Err(field) => positionals.push(Positional::extract(field)?),
            };
        }

        Ok(StructInput {
            name,
            positionals,
            flags,
            info,
        })
    }
}
