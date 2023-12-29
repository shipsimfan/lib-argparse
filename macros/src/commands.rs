use proc_macro_util::{Generator, Parse, Parser, Result, ToTokens};

pub struct Commands {}

impl<'a> Parse<'a> for Commands {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Commands {})
    }
}

impl ToTokens for Commands {
    fn to_tokens(&self, generator: &mut Generator) {}
}
