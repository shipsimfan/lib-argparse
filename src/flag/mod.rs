use std::{borrow::Cow, ops::Deref};

mod action;
mod value;

pub use action::ActionFlag;
pub use value::{SimpleValueParser, Value, ValueParser};

pub enum FlagKind<T, E> {
    Help,
    Action(ActionFlag<T, E>),
    Value(Value<T, E>),
}

pub struct FlagArgument<T, E> {
    short_name: Option<Cow<'static, str>>,
    long_name: Option<Cow<'static, str>>,
    required: bool,
    repeatable: bool,
    kind: FlagKind<T, E>,
}

impl<T, E> FlagArgument<T, E> {
    pub fn new(kind: FlagKind<T, E>) -> Self {
        FlagArgument {
            short_name: None,
            long_name: None,
            required: false,
            repeatable: false,
            kind,
        }
    }

    pub fn short_name(&self) -> Option<&str> {
        self.short_name
            .as_ref()
            .map(|short_name| short_name.deref())
    }

    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_ref().map(|long_name| long_name.deref())
    }

    pub fn required(&self) -> bool {
        self.required
    }

    pub fn repeatable(&self) -> bool {
        self.repeatable
    }

    pub fn kind(&self) -> &FlagKind<T, E> {
        &self.kind
    }

    pub fn set_short_name<S: Into<Cow<'static, str>>>(&mut self, short_name: S) {
        self.short_name = Some(short_name.into())
    }

    pub fn set_long_name<S: Into<Cow<'static, str>>>(&mut self, long_name: S) {
        self.long_name = Some(long_name.into())
    }

    pub fn set_required(&mut self) {
        self.required = true;
    }

    pub fn set_not_required(&mut self) {
        self.required = false;
    }

    pub fn set_repeatable(&mut self) {
        self.repeatable = true;
    }

    pub fn set_not_repeatable(&mut self) {
        self.repeatable = false;
    }

    pub fn set_kind(&mut self, kind: FlagKind<T, E>) {
        self.kind = kind;
    }
}
