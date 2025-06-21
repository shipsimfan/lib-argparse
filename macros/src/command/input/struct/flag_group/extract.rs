use super::FlagGroup;
use proc_macro_util::{
    ast::{items::StructField, AttrInput, SimplePathSegment},
    tokens::{Group, Identifier, Literal},
    Token,
};
use std::borrow::Cow;

impl<'a> FlagGroup<'a> {
    /// Tries to extract a [`FlagGroup`] from `field`, return the field in [`Err`] if the field is not a
    /// flag
    pub fn extract(
        mut field: StructField<'a>,
    ) -> proc_macro_util::Result<Result<Self, StructField<'a>>> {
        let mut flag_group_attribute = None;
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
                "flag_group" => flag_group_attribute = Some(i),
                "flag" => flag_attribute = Some(span),
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

        let flag_group_attribute = match flag_group_attribute {
            Some(index) => field.attributes.swap_remove(index),
            None => return Ok(Err(field)),
        };

        if let Some(span) = flag_attribute {
            return Err(
                span.error("cannot have both `flag` and `flag_group` attributes on a single field")
            );
        }

        if let Some(span) = arg_attribute {
            return Err(
                span.error("cannot have both `arg` and `flag_group` attributes on a single field")
            );
        }

        if let Some(span) = command_attribute {
            return Err(span.error("cannot have the `command` attribute on a member"));
        }

        let group = match flag_group_attribute.attr.input {
            Some(AttrInput::Group(group)) => group,
            None => Cow::Owned(Group::new_parenthesis()),
            Some(AttrInput::Expression(eq, _)) => {
                return Err(eq.spans[0].error("expected a group, not an expression"))
            }
        };

        let mut parser = group.parser();
        let mut header_name = None;
        while !parser.empty() {
            let tag = parser.parse::<Identifier>()?;
            let tag_str = tag.to_string();

            match tag_str.as_str() {
                "name" => {
                    parser.parse::<Token![=]>()?;
                    header_name = Some(parser.parse::<Identifier>()?);
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

        let header_name = Literal::new(
            match header_name {
                Some(header_name) => header_name.to_string(),
                None => field.name.to_string().to_uppercase().replace('_', " "),
            }
            .as_str(),
        );

        Ok(Ok(FlagGroup {
            variable_name: field.name,
            r#type: field.r#type,
            header_name,
        }))
    }
}
