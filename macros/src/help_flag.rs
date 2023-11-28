use crate::flag_name::FlagName;
use proc_macro_util::{to_tokens, Generator, Parse, Parser, Result, ToTokens};

pub struct HelpFlag {
    flag_name: FlagName,
    exit: bool,
}

impl<'a> Parse<'a> for HelpFlag {
    fn parse(parser: &mut Parser<'a>) -> Result<Self> {
        let flag_name = parser.parse()?;

        let exit = if let Some(identifier) = parser.identifier() {
            if identifier != "noexit" {
                return Err(parser.error("expected `noexit` or the end"));
            }

            false
        } else {
            true
        };

        Ok(HelpFlag { flag_name, exit })
    }
}

impl ToTokens for HelpFlag {
    fn to_tokens(&self, generator: &mut Generator) {
        let flag_name = &self.flag_name;

        to_tokens! { generator
            ::argparse::HelpFlagArgument::new()#flag_name
        };

        if !self.exit {
            to_tokens! { generator
                .set_no_exit()
            }
        }
    }
}
