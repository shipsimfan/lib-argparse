use super::PositionalInfo;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for PositionalInfo<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalInfo {
            info_name,
            r#type,
            value,
            min_count,
            max_count,
            default,
            description,
        } = self;

        to_tokens! { generator
            const #info_name: &::argparse::PositionalInfo<#r#type> = &::argparse::PositionalInfo {
                value: #value,
                min_count: #min_count,
                max_count: #max_count,
                default: #default,
                description: #description,
            };
        }
    }
}
