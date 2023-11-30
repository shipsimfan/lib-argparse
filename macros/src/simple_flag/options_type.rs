use proc_macro_util::{ast::Type, to_tokens, Generator, Parse, Parser, Result, ToTokens, Token};

pub struct OptionsType {
    r#type: Type,
}

impl<'a> Parse<'a> for OptionsType {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        parser.parse::<Token![:]>()?;
        let r#type = parser.parse()?;

        Ok(OptionsType { r#type })
    }
}

impl ToTokens for OptionsType {
    fn to_tokens(&self, generator: &mut Generator) {
        let r#type = &self.r#type;

        to_tokens! { generator
            : &mut #r#type
        }
    }
}
