use proc_macro::Delimiter;
use proc_macro_util::{
    ast::{Expression, Punctuated},
    to_tokens, Generator, Parse, Parser, Result, ToTokens, Token,
};

#[derive(Clone)]
pub struct Description<'a> {
    lines: Punctuated<DescriptionLine<'a>, Token![,]>,
}

#[derive(Clone)]
struct DescriptionLine<'a> {
    line: Expression<'a>,
}

impl<'a> Parse<'a> for Description<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let mut lines = Punctuated::new();

        if let Some(group) = parser.group() {
            if group.delimiter() != Delimiter::Bracket {
                return Err(parser.error("expected a list"));
            }

            let mut parser = group.tokens();
            while !parser.empty() {
                lines.push_element(
                    parser
                        .parse()
                        .map_err(|error| error.append("expected a line of description"))?,
                );

                if !parser.peek::<Token![,]>() {
                    break;
                }

                lines.push_separator(parser.parse().unwrap());
            }

            if !parser.empty() {
                return Err(parser.error("unexpected token"));
            }
        } else {
            lines.push_element(
                parser
                    .parse()
                    .map_err(|error| error.append("expected the description"))?,
            );
        }

        Ok(Description { lines })
    }
}

impl<'a> ToTokens for Description<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let lines = &self.lines;

        to_tokens! { generator
            &[#lines]
        }
    }
}

impl<'a> Parse<'a> for DescriptionLine<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let line = parser.parse()?;

        Ok(DescriptionLine { line })
    }
}

impl<'a> ToTokens for DescriptionLine<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let line = &self.line;

        to_tokens! { generator
            &#line
        }
    }
}
