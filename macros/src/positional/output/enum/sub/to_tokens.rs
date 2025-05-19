use super::EnumVariantSub;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for EnumVariantSub<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let EnumVariantSub {
            string,
            r#type,
            name,
        } = self;

        to_tokens! { generator
            #string => match <#r#type as ::argparse::Command>::parse(source, command_list)? {
                Some(value) => {
                    *this = Some(Self::#name(value));
                    Ok(true)
                }
                None => Ok(false),
            }
        }
    }
}

/*
"item2" => match <Command2 as ::argparse::Command>::parse(source, command_list)? {
    Some(value) => {
        *this = Some(Self::Item2(value));
        Ok(true)
    }
    None => Ok(false),
},
*/
