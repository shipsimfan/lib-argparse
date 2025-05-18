use crate::{
    Argument, Error, InvalidLengthError, Positional, PositionalInfo, PositionalResult, Result,
};
use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

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
        T::parse(&mut value, argument, &info.drop_default())?;

        vec.push(value.unwrap());

        if Some(vec.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(Vec::new());

        if Some(vec.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(vec)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}

impl<T: Positional> Positional for VecDeque<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let vec = match this {
            Some(vec) => vec,
            None => {
                *this = Some(VecDeque::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(&mut value, argument, &info.drop_default())?;

        vec.push_back(value.unwrap());

        if Some(vec.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(VecDeque::new());

        if Some(vec.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(vec)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}

impl<T: Positional> Positional for LinkedList<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let list = match this {
            Some(vec) => vec,
            None => {
                *this = Some(LinkedList::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(&mut value, argument, &info.drop_default())?;

        list.push_back(value.unwrap());

        if Some(list.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let list = this.unwrap_or(LinkedList::new());

        if Some(list.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(list)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}

impl<T: Positional + Eq + Hash> Positional for HashSet<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let set = match this {
            Some(vec) => vec,
            None => {
                *this = Some(HashSet::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(&mut value, argument, &info.drop_default())?;

        set.insert(value.unwrap());

        if Some(set.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(HashSet::new());

        if Some(set.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(set)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}

impl<T: Positional + Ord> Positional for BTreeSet<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let set = match this {
            Some(vec) => vec,
            None => {
                *this = Some(BTreeSet::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(&mut value, argument, &info.drop_default())?;

        set.insert(value.unwrap());

        if Some(set.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(BTreeSet::new());

        if Some(set.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(set)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}

impl<T: Positional + Ord> Positional for BinaryHeap<T> {
    fn parse(
        this: &mut Option<Self>,
        argument: Argument,
        info: &PositionalInfo<Self>,
    ) -> PositionalResult {
        let heap = match this {
            Some(vec) => vec,
            None => {
                *this = Some(BinaryHeap::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        T::parse(&mut value, argument, &info.drop_default())?;

        heap.push(value.unwrap());

        if Some(heap.len()) == info.max.map(|f| f as usize) {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let heap = this.unwrap_or(BinaryHeap::new());

        if Some(heap.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_positional_value(
                info.value,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(heap)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max.is_none()
            || (info.max.map(|f| f as usize) != Some(0) && info.max.map(|f| f as usize) != Some(1))
    }
}
