use crate::flag_name::FlagName;
use proc_macro_util::{to_tokens, Generator, Parse, Parser, Result, ToTokens};

pub struct HelpFlag {
    flag_name: FlagName,
}

impl<'a> Parse<'a> for HelpFlag {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser
            .parse()
            .map_err(|error| error.append("expected the flag name"))?;

        Ok(HelpFlag { flag_name })
    }
}

impl ToTokens for HelpFlag {
    fn to_tokens(&self, generator: &mut Generator) {
        let flag_name = &self.flag_name;

        to_tokens! { generator
            ::argparse::HelpFlagArgument::new()#flag_name
        };
    }
}
