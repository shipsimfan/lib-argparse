use super::Flag;
use proc_macro_util::{
    ast::{items::StructField, AttrInput, Expression, SimplePathSegment},
    tokens::{Group, Identifier, Literal},
    Token,
};
use std::borrow::Cow;

impl<'a> Flag<'a> {
    /// Tries to extract a [`Flag`] from `field`, return the field in [`Err`] if the field is not a
    /// flag
    pub fn extract(
        mut field: StructField<'a>,
    ) -> proc_macro_util::Result<Result<Self, StructField<'a>>> {
        let mut flag_attribute = None;
        let mut arg_attribute = None;
        let mut command_attribute = None;
        let mut docs = Vec::new();
        for (i, attribute) in field.attributes.iter().enumerate() {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            let value = attribute.attr.path.first.to_string();
            let span = match &attribute.attr.path.first {
                SimplePathSegment::Identifier(identifier) => identifier.span(),
                SimplePathSegment::Crate(krate) => krate.span,
                SimplePathSegment::DollarCrate(dollar, _) => dollar.spans[0],
                SimplePathSegment::Super(_super) => _super.span,
                SimplePathSegment::_Self(_self) => _self.span,
            };
            match value.as_str() {
                "flag" => flag_attribute = Some(i),
                "arg" => arg_attribute = Some(span),
                "command" => command_attribute = Some(span),
                "doc" => match &attribute.attr.input {
                    Some(AttrInput::Expression(_, expression)) => {
                        docs.push(expression.clone().into_static())
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let flag_attribute = match flag_attribute {
            Some(index) => field.attributes.swap_remove(index),
            None => return Ok(Err(field)),
        };

        if let Some(span) = arg_attribute {
            return Err(
                span.error("cannot have both `arg` and `flag` attributes on a single field")
            );
        }

        if let Some(span) = command_attribute {
            return Err(span.error("cannot have the `command` attribute on a member"));
        }

        let flag_group = match flag_attribute.attr.input {
            Some(AttrInput::Group(group)) => group,
            None => Cow::Owned(Group::new_parenthesis()),
            Some(AttrInput::Expression(eq, _)) => {
                return Err(eq.spans[0].error("expected a group, not an expression"))
            }
        };

        let variable_name = field.name;
        let r#type = field.r#type;

        let variable_name_str = variable_name.to_string();

        let info_name = Identifier::new(&format!("__{}_INFO", variable_name_str.to_uppercase()));
        let mut long_name_str = variable_name_str.replace('_', "-");

        let mut long_name = Literal::new(long_name_str.as_str());
        let mut short_name: Option<Literal> = None;
        let mut value = None;
        let mut min = None;
        let mut max = None;
        let mut default = None;
        let mut description = None;
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
                    short_name = Some(if let Ok(_) = parser.step_parse::<Token![=]>() {
                        parser.parse()?
                    } else {
                        Literal::new(variable_name_str.chars().next().unwrap())
                    });
                }
                "value" => {
                    parser.parse::<Token![=]>()?;
                    value = Some(parser.parse()?);
                }
                "min" => {
                    parser.parse::<Token![=]>()?;
                    min = Some(parser.parse::<Expression>()?.into_static());
                }
                "max" => {
                    parser.parse::<Token![=]>()?;
                    max = Some(parser.parse::<Expression>()?.into_static());
                }
                "default" => {
                    parser.parse::<Token![=]>()?;
                    default = Some(parser.parse::<Expression>()?.into_static());
                }
                "description" => {
                    parser.parse::<Token![=]>()?;

                    description = Some(if let Ok(group) = parser.step_parse::<&Group>() {
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
                }
                _ => return Err(tag.span().error(format!("unknown flag tag \"{tag}\""))),
            }

            match parser.step_parse::<Token![,]>() {
                Ok(_) => {}
                Err(_) => break,
            }
        }

        if !parser.empty() {
            return Err(parser.error("unexpected token"));
        }

        let info_long_name = Literal::new(format!("--{}", long_name_str).as_str());
        let info_short_name = short_name.as_ref().map(|short_name| {
            let short_name_str = short_name.to_string();
            if short_name_str.len() < 2 {
                Literal::new("")
            } else {
                Literal::new(format!("-{}", &short_name_str[1..2]).as_str())
            }
        });

        if description.is_none() && docs.len() > 0 {
            description = Some(docs);
        }

        Ok(Ok(Flag {
            variable_name,
            info_name,
            r#type,
            long_name,
            info_long_name,
            short_name,
            info_short_name,
            value,
            min,
            max,
            default,
            description,
        }))
    }
}
