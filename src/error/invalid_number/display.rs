use crate::{messages::errors::*, InvalidNumberError};
use i18n::translation::m;

impl std::fmt::Display for InvalidNumberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidNumberError::Invalid => m!(NumberInvalid).fmt(f),
            InvalidNumberError::PosOverflow => m!(NumberPosOverflow).fmt(f),
            InvalidNumberError::NegOverflow => m!(NumberNegOverflow).fmt(f),
            InvalidNumberError::Zero => m!(NumberZero).fmt(f),
        }
    }
}
