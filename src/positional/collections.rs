use crate::{Argument, Error, Positional, PositionalInfo, PositionalResult, Result};

impl<T: Positional> Positional for Vec<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let vec = match this {
            Some(vec) => vec,
            None => {
                *this = Some(Vec::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
            },
        )?;

        vec.push(value.unwrap());

        if vec.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(Vec::new());

        if vec.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(vec)
    }
}
