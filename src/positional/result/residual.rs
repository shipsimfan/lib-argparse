use crate::PositionalResult;
use std::ops::Residual;

impl<'a> Residual<Self> for PositionalResult<'a> {
    type TryType = Self;
}
