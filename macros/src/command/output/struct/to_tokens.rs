use super::StructOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for StructOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let StructOutput {
            name,
            generic_params,
            generic_args,
            module_name,
            positional_info,
            positional_declarations,
            positional_matches,
            positional_sub_commands,
            positional_unwraps,
            flag_info,
            flag_declarations,
            flag_long_names,
            flag_short_names,
            flag_unwraps,
            flag_group_declarations,
            flag_group_long_names,
            flag_group_short_names,
            flag_group_unwraps,
            version,
            help,
        } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        let generic_params2 = generic_params.clone();
        let generic_args2 = generic_args.clone();

        to_tokens! { generator
            #[allow(non_snake_case)]
            mod #module_name {
                use super::*;
                impl #generic_params #name #generic_args {
                    // Positional info
                    #positional_info

                    // Flag info
                    #flag_info
                }

                impl #generic_params2 ::argparse::Command for #name2 #generic_args2 {
                    fn parse(__source: &mut dyn ::argparse::ArgumentSource, __command_list: ::std::string::String) -> ::argparse::Result<Option<Self>> {
                        // Positional variables
                        #positional_declarations

                        // Flag variables
                        #flag_declarations

                        // Flag group variables
                        #flag_group_declarations

                        // Accounting variables
                        let mut __current_positional = 0;

                        // Main loop
                        while let Some(__argument) = __source.next() {
                            // Check flags
                            if let Ok(__argument) = __argument.as_str() {
                                if __argument.len() > 2 && __argument.starts_with("--") {
                                    match &__argument[2..] {
                                        #flag_long_names
                                        #version
                                        #help
                                        __flag_name => {
                                            #flag_group_long_names

                                            return Err(::argparse::Error::unknown_argument(__argument.to_string()));
                                        }
                                    }

                                    #[allow(unreachable_code)]
                                    continue;
                                } else if __argument.len() > 1 && __argument != "--" && __argument.starts_with('-') {
                                    let mut __chars = __argument.chars();
                                    __chars.next();
                                    for __c in __chars {
                                        match __c {
                                            #flag_short_names
                                            __flag_name => {
                                                #flag_group_short_names

                                                return Err(::argparse::Error::unknown_argument(::std::format!("-{}", __c)));
                                            }
                                        }
                                    }

                                    continue;
                                }
                            }

                            // Check positionals
                            let __result = match __current_positional {
                                #positional_matches
                                _ => return Err(::argparse::Error::unknown_argument(__argument.to_string())),
                            };

                            #[allow(unreachable_code)]
                            match __result {
                                ::argparse::PositionalResult::Continue => {},
                                ::argparse::PositionalResult::Next => __current_positional += 1,
                                ::argparse::PositionalResult::Error(__error) => return Err(__error),
                                ::argparse::PositionalResult::Sub(__command) => {
                                    match __current_positional {
                                        #positional_sub_commands
                                        _ => unreachable!(),
                                    };
                                }

                            }
                        }

                        // Unwrap values and return result
                        Ok(Some(#name3 {
                            #positional_unwraps
                            #flag_unwraps
                            #flag_group_unwraps
                        }))
                    }
                }
            }
        }
    }
}
