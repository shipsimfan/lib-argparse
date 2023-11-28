use crate::flag_name::FlagName;
use proc_macro_util::{ast::Expression, to_tokens, Generator, Parse, Parser, Result, ToTokens};

pub struct VersionFlag<'a> {
    flag_name: FlagName,
    display: Expression<'a>,
}

impl<'a> Parse<'a> for VersionFlag<'a> {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser.parse()?;
        let display = parser.parse()?;

        Ok(VersionFlag { flag_name, display })
    }
}

impl<'a> ToTokens for VersionFlag<'a> {
    fn to_tokens(&self, generator: &mut Generator) {
        let VersionFlag { flag_name, display } = self;

        to_tokens! { generator
            ::argparse::VersionFlagArgument::new(&#display)#flag_name
        }
    }
}
