use super::Flag;
use proc_macro_util::tokens::Literal;

impl<'a> Flag<'a> {
    /// Gets the short name of this flag
    pub fn short_name(&self) -> Option<&Literal> {
        self.short_name.as_ref()
    }
}
