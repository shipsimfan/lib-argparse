use crate::StrArgument;

impl<'a> std::fmt::Display for StrArgument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_ref().fmt(f)
    }
}
