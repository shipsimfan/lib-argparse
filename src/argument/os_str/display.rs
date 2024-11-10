use crate::OsStrArgument;

impl<'a> std::fmt::Display for OsStrArgument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_string_lossy().fmt(f)
    }
}
