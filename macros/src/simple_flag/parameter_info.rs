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
        let count = parser
            .parse()
            .map_err(|error| error.append("expected the count"))?;
        parser.parse::<Token![*]>()?;
        let hint = parser
            .parse()
            .map_err(|error| error.append("expected the hint"))?;

        let missing = parser
            .parse()
            .map_err(|error| error.append("expected the missing error message"))?;

        Ok(ParameterInfo {
            count,
            hint,
            missing,
        })
    }
}
