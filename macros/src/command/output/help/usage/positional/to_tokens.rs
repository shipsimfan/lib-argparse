use super::PositionalHelpUsageOutput;
use proc_macro_util::{to_tokens, Generator, ToTokens};

impl ToTokens for PositionalHelpUsageOutput {
    fn to_tokens(self, generator: &mut Generator) {
        let PositionalHelpUsageOutput { info_name, value } = self;

        let info_name1 = info_name.clone();

        let value1 = value.clone();
        let value2 = value.clone();
        let value3 = value.clone();

        to_tokens! { generator
            let __required = #info_name.is_required();
            let __multiple = #info_name1.multiple();

            __usage.push_str(match (__required, __multiple) {
                (true, true) => ::std::concat!(#value, ".. "),
                (true, false) => ::std::concat!(#value1, " "),
                (false, true) => ::std::concat!("[", #value2, "..] "),
                (false, false) => ::std::concat!("[", #value3, "] "),
            });
        }
    }
}
