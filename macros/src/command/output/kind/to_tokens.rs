use super::OutputKind;
use proc_macro_util::{Generator, ToTokens};

impl<'a> ToTokens for OutputKind<'a> {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            OutputKind::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
