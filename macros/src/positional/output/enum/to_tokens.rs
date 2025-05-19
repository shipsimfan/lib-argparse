use super::EnumOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl<'a> ToTokens for EnumOutput<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        let EnumOutput {
            name,
            parses,
            expected,
            subs,
            displays,
        } = self;

        let name2 = name.clone();
        let name3 = name.clone();

        to_tokens! { generator
            impl ::argparse::Positional for #name {
                fn parse<'a>(
                    this: &mut Option<Self>,
                    argument: ::argparse::Argument<'a>,
                    info: &::argparse::PositionalInfo<Self>,
                ) -> ::argparse::PositionalResult<'a> {
                    match argument.as_str()? {
                        #parses
                        argument => {
                            ::argparse::PositionalResult::Error(::argparse::Error::invalid_positional_value(
                                info.value,
                                ::argparse::UnexpectedError::new(
                                    argument,
                                    #expected,
                                ),
                            ))
                        }
                    }
                }

                fn sub(
                    this: &mut Option<Self>,
                    command: ::argparse::Argument,
                    source: &mut dyn ::argparse::ArgumentSource,
                    command_list: String,
                ) -> ::argparse::Result<bool> {
                    match command.as_str()? {
                        #subs
                        _ => unimplemented!(),
                    }
                }
            }

            impl ::argparse::DefaultDisplay for #name2 {
                type Display<'a> = &'a Self;

                fn as_display<'a>(&'a self) -> Self::Display<'a> {
                    self
                }
            }

            impl ::std::fmt::Display for #name3 {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        #displays
                    }
                    .fmt(f)
                }
            }
        }
    }
}
