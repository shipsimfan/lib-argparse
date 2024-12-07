use crate::{Argument, Positional, PositionalInfo, PositionalResult};
use std::path::PathBuf;

impl Positional for PathBuf {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        _: &PositionalInfo<Self>,
    ) -> PositionalResult {
        *this = Some(match argument {
            Argument::Str(str) => PathBuf::from(str.into_owned()),
            Argument::OsStr(os_str) => PathBuf::from(os_str.into_owned()),
        });
        PositionalResult::Next
    }
}
