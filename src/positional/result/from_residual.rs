use crate::{Error, PositionalResult};
use std::ops::FromResidual;

impl<'a> FromResidual for PositionalResult<'a> {
    fn from_residual(residual: <Self as std::ops::Try>::Residual) -> Self {
        residual
    }
}

impl<'a, T> FromResidual<Result<T, Error>> for PositionalResult<'a> {
    fn from_residual(residual: Result<T, Error>) -> Self {
        match residual {
            Ok(_) => unimplemented!(),
            Err(error) => PositionalResult::Error(error),
        }
    }
}

impl<'a, T> FromResidual<PositionalResult<'a>> for Result<T, Error> {
    fn from_residual(residual: PositionalResult<'a>) -> Self {
        match residual {
            PositionalResult::Error(error) => Err(error),
            _ => unimplemented!(),
        }
    }
}
