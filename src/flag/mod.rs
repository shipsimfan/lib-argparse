use std::borrow::Cow;

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

    /// The hint displayed in the help
    hint: Option<Cow<'static, str>>,

    /// The description displayed in the help
    description: Cow<'static, str>,

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
    /// Creates a new unnamed `FlagArgument`
    ///
    ///  - `kind` is the kind of flag being created
    ///  - `description` is the description of this argument displayed in the help
    pub fn new<S: Into<Cow<'static, str>>>(kind: FlagKind<T, E>, description: S) -> Self {
        FlagArgument {
            short_name: None,
            long_name: None,
            hint: None,
            description: description.into(),
            required: None,
            repeatable: None,
            kind,

            count: 0,
        }
    }

    /// Returns the name which follows the short prefix
    pub fn short_name(&self) -> Option<&str> {
        self.short_name.as_deref()
    }

    /// Returns the name which follows the long prefix
    pub fn long_name(&self) -> Option<&str> {
        self.long_name.as_deref()
    }

    /// Returns the hint displayed in the help
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }

    /// Returns the description displayed in the help
    pub fn description(&self) -> &str {
        &self.description
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

    /// Sets the name which follows the short prefix
    ///
    ///  - `short_name` is the name which the short name is set to
    pub fn set_short_name<S: Into<Cow<'static, str>>>(&mut self, short_name: S) {
        self.short_name = Some(short_name.into())
    }

    /// Sets the name which follows the long prefix
    ///
    ///  - `long_name` is the name which the long name is set to
    pub fn set_long_name<S: Into<Cow<'static, str>>>(&mut self, long_name: S) {
        self.long_name = Some(long_name.into())
    }

    /// Sets the hint which will be displayed in the help
    ///
    ///  - `hint` is the string which will be displayed
    pub fn set_hint<S: Into<Cow<'static, str>>>(&mut self, hint: S) {
        self.hint = Some(hint.into());
    }

    /// Sets this argument to have no hint in the help
    pub fn clear_hint(&mut self) {
        self.hint = None;
    }

    /// Sets the description which will be displayed in the help
    ///
    ///  - `description` is the string which will be displayed
    pub fn set_description<S: Into<Cow<'static, str>>>(&mut self, description: S) {
        self.description = description.into();
    }

    /// Sets this flag to be required
    ///
    ///  - `missing_error_message` is the error message if this argument is missing in the parse
    pub fn set_required<S: Into<Cow<'static, str>>>(&mut self, missing_error_message: S) {
        self.required = Some(missing_error_message.into());
    }

    /// Sets this flag to be not required
    pub fn set_not_required(&mut self) {
        self.required = None;
    }

    /// Allows this flag to appear more than once
    ///
    ///  - `repeated_error_message` is the error message if this argument is repeated in the parse
    pub fn set_repeatable<S: Into<Cow<'static, str>>>(&mut self, repeated_error_message: S) {
        self.repeatable = Some(repeated_error_message.into());
    }

    /// Only allows this flag to appear once
    pub fn set_not_repeatable(&mut self) {
        self.repeatable = None;
    }

    /// Sets the kind of this flag
    ///
    ///  - `kind` is the kind this flag is set to
    pub fn set_kind(&mut self, kind: FlagKind<T, E>) {
        self.kind = kind;
    }

    /// Parses the flag argument
    ///
    /// Returns `true` if the flag is a help flag
    ///
    ///  - `options` is the developer provided options to be updated
    ///  - `args` is the argument stream to be parsed from
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

    /// Resets this argument's count before a parse and verifies that it is required
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
