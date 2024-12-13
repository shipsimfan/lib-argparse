use super::CommandInfo;
use crate::messages::macros::*;
use i18n::translation::m;
use proc_macro_util::{
    ast::{AttrInput, Expression, OuterAttribute},
    tokens::{Group, Identifier, Literal},
    Error, Token,
};
use std::borrow::Cow;

impl<'a> CommandInfo<'a> {
    /// Extracts the [`CommandInfo`] from `attribute`
    pub fn extract(attribute: OuterAttribute<'a>) -> Result<Self, Error> {
        let group = match attribute.attr.input {
            Some(AttrInput::Group(group)) => group,
            None => Cow::Owned(Group::new_parenthesis()),
            Some(AttrInput::Expression(eq, _)) => {
                return Err(Error::new_at(m!(ExpectedGroupNotExpression), eq.spans[0]))
            }
        };

        let mut parser = group.parser();

        let mut name = None;
        let mut description = None;
        let mut version = None;
        let mut help = false;
        let mut usage_header = None;
        let mut positional_header = None;
        let mut flag_header = None;
        while !parser.empty() {
            let tag = parser.parse::<Identifier>()?;
            let tag_str = tag.to_string();

            match tag_str.as_str() {
                "name" => {
                    parser.parse::<Token![=]>()?;
                    name = Some(parser.parse::<Literal>()?);
                }
                "description" => {
                    parser.parse::<Token![=]>()?;
                    description = Some(parser.parse::<Expression>()?.into_static());
                }
                "version" => {
                    version = Some(if let Ok(_) = parser.step_parse::<Token![=]>() {
                        Some(parser.parse::<Expression>()?.into_static())
                    } else {
                        None
                    });
                }
                "help" => {
                    help = true;
                }
                "usage" => {
                    parser.parse::<Token![=]>()?;
                    usage_header = Some(parser.parse::<Expression>()?.into_static());
                }
                "args" => {
                    parser.parse::<Token![=]>()?;
                    positional_header = Some(parser.parse::<Expression>()?.into_static());
                }
                "flags" => {
                    parser.parse::<Token![=]>()?;
                    flag_header = Some(parser.parse::<Expression>()?.into_static());
                }
                _ => {
                    return Err(Error::new_at(
                        m!(UnknownCommandTag, tag => &tag_str),
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

        Ok(CommandInfo {
            name,
            description,
            version,
            help,
            usage_header,
            positional_header,
            flag_header,
        })
    }
}
