use proc_macro_util::{
    ast::{Expression, Punctuated},
    to_tokens,
    tokens::Comma,
    Delimiter, Parse, ToTokens,
};

pub struct Flags<'a> {
    flags: Punctuated<Expression<'a>, Comma>,
}

impl<'a> Parse<'a> for Flags<'a> {
    fn parse(parser: &mut proc_macro_util::Parser<'a>) -> proc_macro_util::Result<Self> {
        if parser.empty() {
            return Ok(Flags {
                flags: Punctuated::new(),
            });
        }

        let group = parser
            .group()
            .ok_or(parser.error("expected a list of flags"))?;
        if group.delimiter() != Delimiter::Bracket {
            return Err(parser.error("expected square bracket delimiters"));
        }

        todo!()
    }
}

impl<'a> ToTokens for Flags<'a> {
    fn to_tokens(&self, generator: &mut proc_macro_util::Generator) {
        if self.flags.len() == 0 {
            return;
        }

        let flags = &self.flags;

        to_tokens!(generator
            .flags(&[#flags])
        );
    }
}
