use crate::{Argument, Positional, PositionalInfo, PositionalResult};
use std::{ffi::OsString, path::PathBuf};

impl Positional for PathBuf {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        _: &PositionalInfo<Self>,
    ) -> PositionalResult {
        *this = Some(match argument {
            Argument::Str(str) => PathBuf::from(Into::<String>::into(str)),
            Argument::OsStr(os_str) => PathBuf::from(Into::<OsString>::into(os_str)),
        });
        PositionalResult::Next
    }
}
