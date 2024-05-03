use crate::flag_name::FlagName;
use proc_macro_util::{to_tokens, Generator, Parse, Parser, Result, ToTokens};

pub struct ConfigFlag {
    flag_name: FlagName,
}

impl<'a> Parse<'a> for ConfigFlag {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser
            .parse()
            .map_err(|error| error.append("expected the flag name"))?;

        Ok(ConfigFlag { flag_name })
    }
}

impl ToTokens for ConfigFlag {
    fn to_tokens(&self, generator: &mut Generator) {
        let flag_name = &self.flag_name;

        to_tokens! { generator
            ::argparse::ConfigFlagArgument::new()#flag_name
        };
    }
}
