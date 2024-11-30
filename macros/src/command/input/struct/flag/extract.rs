use super::Flag;
use crate::messages::macros::*;
use i18n::translation::m;
use proc_macro_util::{
    ast::{items::StructField, AttrInput, Expression},
    tokens::{Group, Identifier, Literal},
    Error, Token,
};
use std::borrow::Cow;

impl<'a> Flag<'a> {
    /// Tries to extract a [`Flag`] from `field`, return the field in [`Err`] if the field is not a
    /// flag
    pub fn extract(mut field: StructField<'a>) -> Result<Result<Self, StructField<'a>>, Error> {
        let mut flag_attribute = None;
        for (i, attribute) in field.attributes.iter().enumerate() {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            if attribute.attr.path.first.to_string().as_str() == "flag" {
                flag_attribute = Some(i);
                break;
            }
        }

        let flag_attribute = match flag_attribute {
            Some(index) => field.attributes.swap_remove(index),
            None => return Ok(Err(field)),
        };

        let flag_group = match flag_attribute.attr.input {
            Some(AttrInput::Group(group)) => group,
            None => Cow::Owned(Group::new_parenthesis()),
            Some(AttrInput::Expression(eq, _)) => {
                return Err(Error::new_at(m!(ExpectedGroupNotExpression), eq.spans[0]))
            }
        };

        let variable_name = field.name.into_owned();
        let r#type = field.r#type;

        let variable_name_str = variable_name.to_string();

        let info_name = Identifier::new(&format!("__{}_INFO", variable_name_str.to_uppercase()));
        let mut long_name_str = variable_name_str.replace('_', "-");

        let mut long_name = Literal::new(long_name_str.as_str());
        let mut short_name: Option<Literal> = None;
        let mut value = None;
        let mut min_count = Literal::new(0);
        let mut max_count = Literal::new(0);
        let mut default = None;
        let mut parser = flag_group.parser();
        while !parser.empty() {
            let tag = parser.parse::<Identifier>()?;
            let tag_str = tag.to_string();

            match tag_str.as_str() {
                "long_name" => {
                    parser.parse::<Token![=]>()?;
                    long_name = parser.parse()?;
                    long_name_str = long_name.to_string();
                }
                "short_name" => {
                    parser.parse::<Token![=]>()?;
                    short_name = Some(parser.parse()?);
                }
                "value" => {
                    parser.parse::<Token![=]>()?;
                    value = Some(parser.parse()?);
                }
                "min" => {
                    parser.parse::<Token![=]>()?;
                    min_count = parser.parse()?;
                }
                "max" => {
                    parser.parse::<Token![=]>()?;
                    max_count = parser.parse()?;
                }
                "default" => {
                    parser.parse::<Token![=]>()?;
                    default = Some(parser.parse::<Expression>()?.into_static());
                }
                _ => {
                    return Err(Error::new_at(
                        m!(UnknownFlagTag, tag => &tag_str),
                        tag.span(),
                    ))
                }
            }

            match parser.step_parse::<Token![,]>() {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        if !parser.empty() {
            return Err(parser.error(m!(UnexpectedToken)));
        }

        let info_long_name = Literal::new(format!("--{}", long_name_str).as_str());
        let info_short_name = short_name.as_ref().map(|short_name| {
            let short_name_str = short_name.to_string();
            if short_name_str.len() < 2 {
                Literal::new("")
            } else {
                Literal::new(format!("-{}", &short_name_str[1..3]).as_str())
            }
        });

        Ok(Ok(Flag {
            variable_name,
            info_name,
            r#type,
            long_name,
            info_long_name,
            short_name,
            info_short_name,
            value,
            min_count,
            max_count,
            default,
        }))
    }
}
