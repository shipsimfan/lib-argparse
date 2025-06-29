use super::StructOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for StructOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let StructOutput {
            name,
            module_name,
            infos,
            in_progress,
            new_in_progress,
            long_names,
            short_names,
            unwraps,
            usages,
            helps,
            flag_group_in_progress,
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            flag_group_usages,
            flag_group_helps,
        } = self;

        let name2 = name.clone();

        to_tokens! { generator
            #[allow(non_snake_case)]
            mod #module_name {
                use super::*;

                #infos

                impl ::argparse::FlagGroup for #name {
                    type InProgress = (#in_progress #flag_group_in_progress);

                    fn new_in_progress() -> Self::InProgress {
                        (#new_in_progress #flag_group_declarations)
                    }

                    fn parse_long(
                        this: &mut Self::InProgress,
                        flag: &str,
                        source: &mut dyn ::argparse::ArgumentSource,
                    ) -> ::argparse::Result<bool> {
                        match flag {
                            #long_names
                            _ => {
                                #flag_group_long_names

                                return Ok(false)
                            },
                        }
                        #[allow(unreachable_code)]
                        Ok(true)
                    }

                    fn parse_short(
                        this: &mut Self::InProgress,
                        flag: char,
                        source: &mut dyn ::argparse::ArgumentSource,
                    ) -> ::argparse::Result<bool> {
                        match flag {
                            #short_names
                            _ => {
                                #flag_group_short_names

                                return Ok(false)
                            },
                        }
                        #[allow(unreachable_code)]
                        Ok(true)
                    }

                    fn unwrap(this: Self::InProgress) -> ::argparse::Result<Self> {
                        Ok(#name2 {
                            #unwraps
                            #flag_group_unwraps
                        })
                    }

                    fn print_help_usage() -> bool {
                        #[allow(unused_mut)]
                        let mut __optional_flags = false;
                        #usages
                        #flag_group_usages
                        __optional_flags
                    }

                    fn print_help() {
                        #helps
                        #flag_group_helps
                    }
                }
            }
        }
    }
}
