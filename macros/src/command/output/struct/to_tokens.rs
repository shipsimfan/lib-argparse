use super::StructOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for StructOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let StructOutput { name } = self;

        to_tokens! { generator
            // Positional info

            // Flag info

            // Positional variables

            // Flag variables

            // Accounting variables
            let mut __current_positional = 0;

            // Main loop
            while let Some(__argument) = __source.next() {
                // Check flags
                if let Ok(__argument) = __argument.as_str() {
                    if __argument.len() > 2 && __argument.starts_with("--") {
                        match &__argument[2..] {
                            _ => return Err(::argparse::Error::unknown_argument(__argument.to_string())),
                        }

                        #[allow(unreachable_code)]
                        continue;
                    } else if __argument.len() > 1 && __argument != "--" && __argument.starts_with('-') {
                        let mut __chars = __argument.chars();
                        __chars.next();
                        for __c in __chars {
                            match __c {
                                _ => return Err(::argparse::Error::unknown_argument(::std::format!("-{}", __c))),
                            }
                        }

                        continue;
                    }
                }

                // Check positionals
                let __result = match __current_positional {
                    _ => return Err(::argparse::Error::unknown_argument(__argument.to_string())),
                };

                #[allow(unreachable_code)]
                match __result {
                    ::argparse::PositionalResult::Continue => {},
                    ::argparse::PositionalResult::Next => __current_positional += 1,
                    ::argparse::PositionalResult::Error(__error) => return Err(__error),
                    ::argparse::PositionalResult::Sub => {
                        match __current_positional {
                            _ => unreachable!(),
                        }?;

                        break;
                    }

                }
            }

            // Unwrap positionals

            // Unwrap flags

            // Return result
            Ok(#name {})
        }
    }
}
