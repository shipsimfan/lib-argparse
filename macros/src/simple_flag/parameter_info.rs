use proc_macro_util::{ast::Expression, tokens::Literal, Parse, Parser, Result, Token};

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

        let missing = parser.parse()?;

        Ok(ParameterInfo {
            count,
            hint,
            missing,
        })
    }
}
