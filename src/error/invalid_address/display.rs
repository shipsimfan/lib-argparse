use crate::{messages::errors::*, InvalidAddressError};
use i18n::translation::m;

impl std::fmt::Display for InvalidAddressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        m!(AddressInvalid).fmt(f)
    }
}
