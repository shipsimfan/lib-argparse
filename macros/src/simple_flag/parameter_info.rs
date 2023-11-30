use proc_macro_util::{ast::Expression, tokens::Literal, Delimiter, Parse, Parser, Result, Token};

pub struct ParameterInfo<'a> {
    count: Literal,
    hint: Literal,
    missing: Expression<'a>,
}

impl<'a> ParameterInfo<'a> {
    pub fn count(&self) -> &Literal {
        &self.count
    }

    pub fn hint(&self) -> &Literal {
        &self.hint
    }

    pub fn missing(&self) -> &Expression<'a> {
        &self.missing
    }
}

impl<'a> Parse<'a> for ParameterInfo<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let count = parser.parse()?;
        parser.parse::<Token![*]>()?;
        let hint = parser.parse()?;

        let missing = if let Some(group) = parser.group() {
            if group.delimiter() != Delimiter::Parenthesis {
                return Err(parser.error("expected the missing error message"));
            }
            let mut group_parser = group.tokens();

            let missing = group_parser.parse()?;

            if !group_parser.empty() {
                return Err(parser.error("unexpected tokens"));
            }

            missing
        } else {
            return Err(parser.error("expected the missing error message"));
        };

        Ok(ParameterInfo {
            count,
            hint,
            missing,
        })
    }
}
