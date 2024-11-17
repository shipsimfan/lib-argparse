use crate::{Argument, Positional, PositionalInfo, PositionalResult};
use std::ffi::OsString;

impl Positional for String {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        _: &PositionalInfo<Self>,
    ) -> PositionalResult {
        *this = Some(match argument {
            Argument::Str(str) => str.into_owned(),
            Argument::OsStr(os_str) => match os_str.as_str() {
                Ok(string) => string.to_owned(),
                Err(error) => return PositionalResult::Error(error),
            },
        });
        PositionalResult::Next
    }
}

impl Positional for OsString {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        _: &PositionalInfo<Self>,
    ) -> PositionalResult {
        *this = Some(match argument {
            Argument::Str(str) => OsString::from(str.as_ref()),
            Argument::OsStr(os_str) => os_str.to_os_string(),
        });
        PositionalResult::Next
    }
}
