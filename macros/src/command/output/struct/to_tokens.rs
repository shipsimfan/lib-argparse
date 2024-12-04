use super::StructOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for StructOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let StructOutput {
            name,
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
        } = self;

        to_tokens! { generator
            // Positional info
            #positional_info

            // Flag info
            #flag_info

            // Positional variables
            #positional_declarations

            // Flag variables
            #flag_declarations

            // Accounting variables
            let mut __current_positional = 0;

            // Main loop
            while let Some(__argument) = __source.next() {
                // Check flags
                if let Ok(__argument) = __argument.as_str() {
                    if __argument.len() > 2 && __argument.starts_with("--") {
                        match &__argument[2..] {
                            #flag_long_names
                            _ => return Err(::argparse::Error::unknown_argument(__argument.to_string())),
                        }

                        #[allow(unreachable_code)]
                        continue;
                    } else if __argument.len() > 1 && __argument != "--" && __argument.starts_with('-') {
                        let mut __chars = __argument.chars();
                        __chars.next();
                        for __c in __chars {
                            match __c {
                                #flag_short_names
                                _ => return Err(::argparse::Error::unknown_argument(::std::format!("-{}", __c))),
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
                    ::argparse::PositionalResult::Sub => {
                        match __current_positional {
                            #positional_sub_commands
                            _ => unreachable!(),
                        };
                    }

                }
            }

            // Unwrap values and return result
            Ok(Some(#name {
                #positional_unwraps
                #flag_unwraps
            }))
        }
    }
}
