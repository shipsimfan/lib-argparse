use crate::{Error, PositionalResult};
use std::ops::FromResidual;

impl FromResidual for PositionalResult {
    fn from_residual(residual: <Self as std::ops::Try>::Residual) -> Self {
        residual
    }
}

impl<T> FromResidual<Result<T, Error>> for PositionalResult {
    fn from_residual(residual: Result<T, Error>) -> Self {
        match residual {
            Ok(_) => unimplemented!(),
            Err(error) => PositionalResult::Error(error),
        }
    }
}

impl<T> FromResidual<PositionalResult> for Result<T, Error> {
    fn from_residual(residual: PositionalResult) -> Self {
        match residual {
            PositionalResult::Error(error) => Err(error),
            _ => unimplemented!(),
        }
    }
}
