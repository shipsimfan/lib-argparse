use crate::{FlagArgument, FlagKind};

/// Flag which calls an function upon matching
pub struct ActionFlag<T, E> {
    /// The function to be called upon matching
    action: Box<dyn Fn(&mut T, Vec<String>) -> Result<(), E>>,

    /// The number of required arguments for the action
    count: usize,
}

impl<T, E> ActionFlag<T, E> {
    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T)`
    pub fn simple(action: impl Fn(&mut T) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| Ok(action(options))),
            count: 0,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T) -> Result<(), E>`
    pub fn simple_res(action: impl Fn(&mut T) -> Result<(), E> + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, _| action(options)),
            count: 0,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, &str)`
    pub fn str(action: impl Fn(&mut T, &str) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, &strings[0]))),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, &str) -> Result<(), E>`
    pub fn str_res(action: impl Fn(&mut T, &str) -> Result<(), E> + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, &strings[0])),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, String)`
    pub fn string(action: impl Fn(&mut T, String) + 'static) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| {
                Ok(action(options, strings.swap_remove(0)))
            }),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, String) -> Result<(), E>`
    pub fn string_res(
        action: impl Fn(&mut T, String) -> Result<(), E> + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, mut strings| action(options, strings.swap_remove(0))),
            count: 1,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, Vec<String>)`
    pub fn vec_string(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| Ok(action(options, strings))),
            count,
        }))
    }

    /// Creates and `ActionFlag` with `action`
    ///
    /// `action` is in the form `fn(&mut T, Vec<String>) -> Result<(), E>`
    pub fn vec_string_res(
        count: usize,
        action: impl Fn(&mut T, Vec<String>) -> Result<(), E> + 'static,
    ) -> FlagArgument<T, E> {
        FlagArgument::new(FlagKind::Action(ActionFlag {
            action: Box::new(move |options, strings| action(options, strings)),
            count,
        }))
    }
}
