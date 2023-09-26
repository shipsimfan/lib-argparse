use crate::{FlagArgument, FlagKind};

pub struct ActionFlag<T, E> {
    action: Box<dyn Fn(&mut T, Vec<String>) -> Result<(), E>>,
    count: usize,
}

impl<T, E> ActionFlag<T, E> {
    pub fn simple(action: impl Fn(&mut T) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| Ok(action(options))),
            count: 0,
        }))
    }

    pub fn simple_res(action: impl Fn(&mut T) -> Result<(), E> + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| action(options)),
            count: 0,
        }))
    }

    pub fn str(action: impl Fn(&mut T, &str) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, &strings[0]))),
            count: 1,
        }))
    }

    pub fn str_res(action: impl Fn(&mut T, &str) -> Result<(), E> + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, &strings[0])),
            count: 1,
        }))
    }

    pub fn string(action: impl Fn(&mut T, String) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| {
                Ok(action(options, strings.swap_remove(0)))
            }),
            count: 1,
        }))
    }

    pub fn string_res(
        action: impl Fn(&mut T, String) -> Result<(), E> + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| action(options, strings.swap_remove(0))),
            count: 1,
        }))
    }

    pub fn vec_string(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, strings))),
            count,
        }))
    }

    pub fn vec_string_res(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) -> Result<(), E> + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, strings)),
            count,
        }))
    }

    pub fn action(&self) -> &dyn Fn(&mut T, Vec<String>) -> Result<(), E> {
        &self.action
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
