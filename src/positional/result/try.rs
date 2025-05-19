use crate::PositionalResult;
use std::ops::{ControlFlow, Try};

impl<'a> Try for PositionalResult<'a> {
    type Output = Self;

    type Residual = Self;

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            PositionalResult::Error(_) => ControlFlow::Break(self),
            _ => ControlFlow::Continue(self),
        }
    }

    fn from_output(output: Self::Output) -> Self {
        output
    }
}
