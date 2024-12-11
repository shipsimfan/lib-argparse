use super::FlagInfo;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for FlagInfo<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let FlagInfo {
            info_name,
            r#type,
            info_long_name: long_name,
            info_short_name: short_name,
            value,
            min_count,
            max_count,
            default,
        } = self;

        to_tokens! { generator
            const #info_name: &::argparse::FlagInfo<#r#type> = &::argparse::FlagInfo {
                long_name: #long_name,
                short_name: #short_name,
                value: #value,
                min_count: #min_count,
                max_count: #max_count,
                default: #default,
                description: None,
            };
        }
    }
}
