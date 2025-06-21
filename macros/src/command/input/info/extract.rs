use super::CommandInfo;
use proc_macro_util::{
    ast::{AttrInput, Expression, OuterAttribute},
    tokens::{Group, Identifier, Literal},
    Result, Token,
};
use std::borrow::Cow;

impl<'a> CommandInfo<'a> {
    /// Extracts the [`CommandInfo`] from `attribute`
    pub fn extract(attribute: OuterAttribute<'a>, docs: Vec<Expression<'a>>) -> Result<Self> {
        let group = match attribute.attr.input {
            Some(AttrInput::Group(group)) => group,
            None => Cow::Owned(Group::new_parenthesis()),
            Some(AttrInput::Expression(eq, _)) => {
                return Err(eq.spans[0].error("expected a group, not an expression"))
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
                    description = Some(if parser.step_parse::<Token![=]>().is_ok() {
                        Some(if let Ok(group) = parser.step_parse::<&Group>() {
                            let mut parser = group.parser();
                            let mut description = vec![parser.parse::<Expression>()?.into_static()];
                            while !parser.empty() {
                                parser.parse::<Token![,]>()?;
                                if parser.empty() {
                                    break;
                                }

                                description.push(parser.parse::<Expression>()?.into_static());
                            }
                            description
                        } else {
                            vec![parser.parse::<Expression>()?.into_static()]
                        })
                    } else {
                        None
                    })
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
                    return Err(tag
                        .span()
                        .error(format!("unknown command tag \"{tag_str}\"")))
                }
            }

            match parser.step_parse::<Token![,]>() {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        let description = match description {
            Some(Some(description)) => Some(description),
            Some(None) => Some(docs),
            None => None,
        };

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
