use proc_macro_util::{Generator, Parse, Parser, Result, ToTokens};

pub struct Command {}

impl<'a> Parse<'a> for Command {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        Ok(Command {})
    }
}

impl ToTokens for Command {
    fn to_tokens(&self, generator: &mut Generator) {}
}
