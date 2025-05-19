use crate::{ArgumentSource, Error, Flag, FlagInfo, InvalidLengthError, Result};
use std::{
    collections::{BTreeSet, BinaryHeap, HashSet, LinkedList, VecDeque},
    hash::Hash,
};

impl<T: Flag> Flag for Vec<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let vec = match this {
            Some(vec) => vec,
            None => {
                *this = Some(Vec::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        vec.push(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < vec.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(Vec::new());

        if Some(vec.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(vec)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}

impl<T: Flag> Flag for VecDeque<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let vec = match this {
            Some(vec) => vec,
            None => {
                *this = Some(VecDeque::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        vec.push_back(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < vec.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let vec = this.unwrap_or(VecDeque::new());

        if Some(vec.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(vec)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}

impl<T: Flag> Flag for LinkedList<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let list = match this {
            Some(vec) => vec,
            None => {
                *this = Some(LinkedList::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        list.push_back(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < list.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let list = this.unwrap_or(LinkedList::new());

        if Some(list.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(list)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}

impl<T: Flag + Eq + Hash> Flag for HashSet<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let set = match this {
            Some(vec) => vec,
            None => {
                *this = Some(HashSet::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        set.insert(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < set.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(HashSet::new());

        if Some(set.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(set)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}

impl<T: Flag + Ord> Flag for BTreeSet<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let set = match this {
            Some(vec) => vec,
            None => {
                *this = Some(BTreeSet::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        set.insert(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < set.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let set = this.unwrap_or(BTreeSet::new());

        if Some(set.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(set)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}

impl<T: Flag + Ord> Flag for BinaryHeap<T> {
    fn parse(
        this: &mut Option<Self>,
        source: &mut dyn ArgumentSource,
        info: &FlagInfo<Self>,
        long: bool,
    ) -> Result<()> {
        let heap = match this {
            Some(vec) => vec,
            None => {
                *this = Some(BinaryHeap::new());
                this.as_mut().unwrap()
            }
        };

        let mut value = None;
        let info = FlagInfo {
            long_name: info.long_name,
            short_name: info.short_name,
            value: info.value,
            min: None,
            max: None,
            default: None,
            description: None,
        };
        T::parse(&mut value, source, &info, long)?;

        heap.push(T::unwrap(value, &info)?);

        if info.max.map(|f| (f as usize) < heap.len()).unwrap_or(false) {
            return Err(Error::invalid_flag_value(
                &info,
                long,
                InvalidLengthError::TooLong,
            ));
        }

        Ok(())
    }

    fn unwrap(this: Option<Self>, info: &FlagInfo<Self>) -> Result<Self> {
        let heap = this.unwrap_or(BinaryHeap::new());

        if Some(heap.len()) < info.min.map(|f| f as usize) {
            return Err(Error::invalid_flag_value(
                info,
                true,
                InvalidLengthError::TooShort,
            ));
        }

        Ok(heap)
    }

    fn is_required(info: &FlagInfo<Self>) -> bool {
        info.min.map(|f| f as usize > 0).unwrap_or(false)
    }
}
