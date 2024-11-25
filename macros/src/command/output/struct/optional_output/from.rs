use super::OptionalOutput;
use proc_macro_util::ToTokens;

impl<T: ToTokens> From<Option<T>> for OptionalOutput<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => OptionalOutput::Some(value),
            None => OptionalOutput::None,
        }
    }
}

impl<T: ToTokens> From<T> for OptionalOutput<T> {
    fn from(value: T) -> Self {
        OptionalOutput::Some(value)
    }
}
