use std::borrow::Cow;

use crate::{FlagArgument, FlagKind};

mod action;
mod help;
mod value;

#[macro_export]
/// Creates a [`FlagArgument`]
///
///  - `short_name` is the short name
///  - `long_name` is the long name
///  - `hint` is the usage hint displayed in the help
///  - `description` is the description displayed in the help
///  - `kind` is the flag kind
macro_rules! flag {
    ($short_name: literal $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::None,
            $description.into(),
            $kind,
        )
    };

    (, $long_name: literal $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
            $description.into(),
            $kind,
        )
    };

    ($short_name: literal, $long_name: literal $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
            $description.into(),
            $kind,
        )
    };

    ($short_name: literal $hint: literal $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
            $description.into(),
            $kind,
        )
    };

    (, $long_name: literal $hint: literal $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
            $description.into(),
            $kind,
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal  $description: literal => $kind: expr) => {
        $crate::macros::__flag(
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
            $description.into(),
            $kind,
        )
    };
}

#[doc(hidden)]
/// Creates a [`FlagArgument`]
///
///  - `short_name` is the short name
///  - `long_name` is the long name
///  - `hint` is the usage hint displayed in the help
///  - `description` is the description displayed in the help
///  - `kind` is the flag kind
pub fn __flag<T, E>(
    short_name: Option<Cow<'static, str>>,
    long_name: Option<Cow<'static, str>>,
    hint: Option<Cow<'static, str>>,
    description: Cow<'static, str>,
    kind: FlagKind<T, E>,
) -> FlagArgument<T, E> {
    __name_flag(
        FlagArgument::new(kind, description),
        short_name,
        long_name,
        hint,
    )
}

#[doc(hidden)]
/// Creates a [`FlagArgument`]
///
///  - `flag` is the flag argument being modified
///  - `short_name` is the short name
///  - `long_name` is the long name
///  - `hint` is the usage hint displayed in the help
pub fn __name_flag<T, E>(
    mut flag: FlagArgument<T, E>,
    short_name: Option<Cow<'static, str>>,
    long_name: Option<Cow<'static, str>>,
    hint: Option<Cow<'static, str>>,
) -> FlagArgument<T, E> {
    flag = match short_name {
        Some(short_name) => flag.set_short_name(short_name),
        None => flag,
    };

    flag = match long_name {
        Some(long_name) => flag.set_long_name(long_name),
        None => flag,
    };

    match hint {
        Some(hint) => flag.set_hint(hint),
        None => flag,
    }
}
