use crate::Argument;

impl<'a> std::fmt::Display for Argument<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::OsStr(os) => os.fmt(f),
            Argument::Str(str) => str.fmt(f),
        }
    }
}
