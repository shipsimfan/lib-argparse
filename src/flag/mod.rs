use std::{borrow::Cow, ops::Deref};

mod action;
mod set;
mod value;

pub(crate) use set::FlagSet;

pub use action::ActionFlag;
pub use value::{SimpleValueParser, ValueFlag, ValueParser};

use crate::{ArgStream, Error};

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
    ///
    /// The contained string is the message displayed if missing
    required: Option<Cow<'static, str>>,

    /// Determines if this flag can appear more than once
    ///
    /// The contained string is the message displayed upon repeat
    repeatable: Option<Cow<'static, str>>,

    /// The type of this flag
    kind: FlagKind<T, E>,

    /// The number of times this argument has been used in the current parse
    count: usize,
}

impl<T, E> FlagArgument<T, E> {
    /// Creates a new unnamed `FlagArgument` of the specified kind
    pub fn new(kind: FlagKind<T, E>) -> Self {
        FlagArgument {
            short_name: None,
            long_name: None,
            required: None,
            repeatable: None,
            kind,

            count: 0,
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
    ///
    /// Contained string is the error message if the argument is missing
    pub fn required(&self) -> Option<&str> {
        self.required.as_deref()
    }

    /// Can this flag appear more than once
    ///
    /// Contained string is the error message if the argument is repeated
    pub fn repeatable(&self) -> Option<&str> {
        self.repeatable.as_deref()
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
    ///
    /// `missing_error_message` is the error message if this argument is missing in the parse
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, missing_error_message: S) {
        self.required = Some(missing_error_message.into());
    }

    /// Sets this flag to be not required
    pub fn set_not_required(&mut self) {
        self.required = None;
    }

    /// Allows this flag to appear more than once
    ///
    /// `repeated_error_message` is the error message if this argument is repeated in the parse
    pub fn set_repeatable<S: Into<Cow<'static, str>>>(&mut self, repeated_error_message: S) {
        self.repeatable = Some(repeated_error_message.into());
    }

    /// Only allows this flag to appear once
    pub fn set_not_repeatable(&mut self) {
        self.repeatable = None;
    }

    /// Sets the type of this flag to `kind`
    pub fn set_kind(&mut self, kind: FlagKind<T, E>) {
        self.kind = kind;
    }

    /// Parses the flag argument
    ///
    /// Returns `true` if the flag is a help flag
    pub(crate) fn parse(
        &mut self,
        options: &mut T,
        args: &mut ArgStream,
    ) -> Result<bool, Error<E>> {
        if let Some(error_message) = &self.repeatable {
            if self.count >= 1 {
                return Err(Error::RepeatedArgument(error_message.clone()));
            }
        }

        self.count += 1;

        match &mut self.kind {
            FlagKind::Help => return Ok(true),
            FlagKind::Action(action_flag) => action_flag.parse(options, args),
            FlagKind::Value(value_flag) => value_flag.parse(options, args),
        }
        .map(|_| false)
    }

    /// Resets this arguments count before a parse
    pub(crate) fn finalize(&mut self) -> Result<(), Error<E>> {
        if let Some(error_message) = &self.required {
            if self.count == 0 {
                return Err(Error::MissingArgument(error_message.clone()));
            }
        }

        self.count = 0;

        Ok(())
    }
}
