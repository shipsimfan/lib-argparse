use super::PositionalHelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalHelpUsageOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalHelpUsageOutput { info_name, value } = self;

        let info_name1 = info_name.clone();

        to_tokens! { generator
            let __required = #info_name.is_required();
            let __multiple = #info_name1.multiple();

            if !__required {
                ::std::print!("[");
            }

            ::std::print!("{}", #value);

            if __multiple {
                ::std::print!("..");
            }

            if !__required {
                ::std::print!("]");
            }

            ::std::print!(" ");
        }
    }
}
