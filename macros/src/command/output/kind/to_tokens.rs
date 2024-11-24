use super::OutputKind;
use proc_macro_util::{Generator, ToTokens};

impl ToTokens for OutputKind {
    fn to_tokens(self, generator: &mut Generator) {
        match self {
            OutputKind::Struct(r#struct) => r#struct.to_tokens(generator),
        }
    }
}
