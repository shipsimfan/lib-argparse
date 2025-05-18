use super::Positional;
use proc_macro_util::{
    ast::{items::StructField, AttrInput, Expression},
    tokens::{Group, Identifier, Literal},
    Error, Token,
};

impl<'a> Positional<'a> {
    pub fn extract(field: StructField<'a>) -> Result<Self, Error> {
        let variable_name = field.name.into_owned();
        let r#type = field.r#type;

        let name_upper = variable_name.to_string().to_uppercase();
        let info_name = Identifier::new(&format!("__{}_INFO", name_upper));

        let mut arg_attribute = None;
        let mut command_attribute = false;
        let mut docs = Vec::new();
        for attribute in field.attributes {
            if attribute.attr.path.remaining.len() > 0 || attribute.attr.path.leading.is_some() {
                continue;
            }

            match attribute.attr.path.first.to_string().as_str() {
                "arg" => match attribute.attr.input {
                    Some(AttrInput::Expression(eq, _)) => {
                        return Err(Error::new_at(
                            "expected a group, not an expression",
                            eq.spans[0],
                        ))
                    }
                    Some(AttrInput::Group(group)) => arg_attribute = Some(group),
                    None => {}
                },
                "command" => command_attribute = true,
                "doc" => match attribute.attr.input {
                    Some(AttrInput::Expression(_, expression)) => {
                        docs.push(expression.into_static())
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if command_attribute {
            return Err(Error::new(
                "cannot have the `command` attribute on a member",
            ));
        }

        let mut value = Literal::new(name_upper.replace('_', "-").as_str());
        let mut min = None;
        let mut max = None;
        let mut default = None;
        let mut description = None;
        if let Some(attribute) = arg_attribute {
            let mut parser = attribute.parser();
            while !parser.empty() {
                let tag = parser.parse::<Identifier>()?;
                let tag_str = tag.to_string();

                match tag_str.as_str() {
                    "value" => {
                        parser.parse::<Token![=]>()?;
                        value = parser.parse()?;
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
                    _ => {
                        return Err(Error::new_at(
                            format!("unknown arg tag \"{tag_str}\""),
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
                return Err(parser.error("unexpected token"));
            }
        }

        if description.is_none() && docs.len() > 0 {
            description = Some(docs);
        }

        Ok(Positional {
            variable_name,
            info_name,
            r#type,
            value,
            min,
            max,
            default,
            description,
        })
    }
}
