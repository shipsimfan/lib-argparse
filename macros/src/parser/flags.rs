use proc_macro_util::{
    ast::{Expression, Punctuated},
    to_tokens, Delimiter, Parse, ToTokens, Token,
};

pub struct Flags<'a> {
    flags: Punctuated<Expression<'a>, Token![,]>,
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

        let mut flags = Punctuated::new();
        let mut parser = group.tokens();
        while !parser.empty() {
            flags.push_element(parser.parse()?);

            if parser.peek::<Token![,]>() {
                flags.push_seperator(parser.parse()?);
            } else {
                break;
            }
        }

        if parser.empty() {
            Ok(Flags { flags })
        } else {
            Err(parser.error("expected a ',' or the end of the flags"))
        }
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
