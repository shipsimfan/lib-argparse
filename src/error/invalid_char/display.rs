use crate::{messages::errors::*, InvalidCharError};
use i18n::translation::m;

impl std::fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        m!(CharInvalid).fmt(f)
    }
}
