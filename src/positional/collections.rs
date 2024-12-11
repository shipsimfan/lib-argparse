use crate::{Argument, Error, Positional, PositionalInfo, PositionalResult, Result};
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
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

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
            },
        )?;

        vec.push_back(value.unwrap());

        if vec.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(VecDeque::new());

        if vec.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(vec)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
            },
        )?;

        list.push_back(value.unwrap());

        if list.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let list = this.unwrap_or(LinkedList::new());

        if list.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(list)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
            },
        )?;

        set.insert(value.unwrap());

        if set.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(HashSet::new());

        if set.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(set)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
            },
        )?;

        set.insert(value.unwrap());

        if set.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(BTreeSet::new());

        if set.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(set)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
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
        T::parse(
            &mut value,
            argument,
            &PositionalInfo {
                value: info.value,
                min_count: 1,
                max_count: 1,
                default: None,
                description: None,
            },
        )?;

        heap.push(value.unwrap());

        if heap.len() == info.max_count {
            PositionalResult::Next
        } else {
            PositionalResult::Continue
        }
    }

    fn unwrap(this: Option<Self>, info: &PositionalInfo<Self>) -> Result<Self> {
        let heap = this.unwrap_or(BinaryHeap::new());

        if heap.len() < info.min_count {
            return Err(Error::missing_positional_value(info.value));
        }

        Ok(heap)
    }

    fn is_required(info: &PositionalInfo<Self>) -> bool {
        info.min_count > 0
    }

    fn multiple(info: &PositionalInfo<Self>) -> bool {
        info.max_count != 1
    }
}
