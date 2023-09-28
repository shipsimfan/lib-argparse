use std::{borrow::Cow, ops::Deref};

mod action;
mod set;
mod value;

pub(crate) use set::FlagSet;

pub use action::ActionFlag;
pub use value::{SimpleValueParser, ValueFlag, ValueParser};

/// The type of a flag
pub enum FlagKind<T, E: 'static> {
    Help,
    Action(ActionFlag<T, E>),
    Value(ValueFlag<T, E>),
}

/// An argument that starts with the short or long prefix
pub struct FlagArgument<T, E: 'static> {
    /// The name following the short prefix
    short_name: Option<Cow<'static, str>>,
    /// The name following the long prefix
    long_name: Option<Cow<'static, str>>,
    /// Determines if this flag is required
    required: bool,
    /// Determines if this flag can appear more than once
    repeatable: bool,
    /// The type of this flag
    kind: FlagKind<T, E>,
}

impl<T, E> FlagArgument<T, E> {
    /// Creates a new unnamed `FlagArgument` of the specified kind
    pub fn new(kind: FlagKind<T, E>) -> Self {
        FlagArgument {
            short_name: None,
            long_name: None,
            required: false,
            repeatable: false,
            kind,
        }
    }

    /// Returns the name which follows the short prefix
    pub fn short_name(&self) -> Option<&str> {
        self.short_name
            .as_ref()
            .map(|short_name| short_name.deref())
    }

    /// Returns the name which follows the long prefix
    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_ref().map(|long_name| long_name.deref())
    }

    /// Is this flag required
    pub fn required(&self) -> bool {
        self.required
    }

    /// Can this flag appear more than once
    pub fn repeatable(&self) -> bool {
        self.repeatable
    }

    /// Returns the type of this flag
    pub fn kind(&self) -> &FlagKind<T, E> {
        &self.kind
    }

    /// Sets the name which follows the short prefix to `short_name`
    pub fn set_short_name<S: Into<Cow<'static, str>>>(&mut self, short_name: S) {
        self.short_name = Some(short_name.into())
    }

    /// Sets the name which follows the long prefix to `long_name`
    pub fn set_long_name<S: Into<Cow<'static, str>>>(&mut self, long_name: S) {
        self.long_name = Some(long_name.into())
    }

    /// Sets this flag to be required
    pub fn set_required(&mut self) {
        self.required = true;
    }

    /// Sets this flat to be not required
    pub fn set_not_required(&mut self) {
        self.required = false;
    }

    /// Allows this flag to appear more than once
    pub fn set_repeatable(&mut self) {
        self.repeatable = true;
    }

    /// Only allows this flag to appear once
    pub fn set_not_repeatable(&mut self) {
        self.repeatable = false;
    }

    /// Sets the type of this flag to `kind`
    pub fn set_kind(&mut self, kind: FlagKind<T, E>) {
        self.kind = kind;
    }
}
