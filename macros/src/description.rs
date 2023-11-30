use proc_macro::Delimiter;
use proc_macro_util::{
    ast::Punctuated, to_tokens, tokens::Literal, Generator, Parse, Parser, Result, ToTokens, Token,
};

pub struct Description {
    lines: Punctuated<DescriptionLine, Token![,]>,
}

struct DescriptionLine {
    line: Literal,
}

impl<'a> Parse<'a> for Description {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut lines = Punctuated::new();

        if let Some(group) = parser.group() {
            if group.delimiter() != Delimiter::Bracket {
                return Err(parser.error("expected a list"));
            }

            let mut parser = group.tokens();
            while !parser.empty() {
                lines.push_element(parser.parse()?);

                if !parser.peek::<Token![,]>() {
                    break;
                }

                lines.push_seperator(parser.parse()?);
            }

            if !parser.empty() {
                return Err(parser.error("unexpected token"));
            }
        } else {
            lines.push_element(parser.parse()?);
        }

        Ok(Description { lines })
    }
}

impl ToTokens for Description {
    fn to_tokens(&self, generator: &mut Generator) {
        let lines = &self.lines;

        to_tokens! { generator
            &[#lines]
        }
    }
}

impl<'a> Parse<'a> for DescriptionLine {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let line = parser.parse()?;

        Ok(DescriptionLine { line })
    }
}

impl ToTokens for DescriptionLine {
    fn to_tokens(&self, generator: &mut Generator) {
        let line = &self.line;

        to_tokens! { generator
            &#line
        }
    }
}
