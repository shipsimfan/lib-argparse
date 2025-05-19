use crate::UnexpectedError;

impl std::fmt::Display for UnexpectedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unexpected \"{}\", expected {}",
            self.unexpected, self.expected
        )
    }
}
