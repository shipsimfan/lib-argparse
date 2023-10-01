#[allow(unused_imports)]
use crate::{FlagArgument, ValueParser};

#[macro_export]
/// Creates a [`FlagArgument`]
///
///  - `short_name` is the short name
///  - `long_name` is the long name
///  - `hint` is the usage hint displayed in the help
///  - `description` is the description displayed in the help
///  - `parser` is the [`ValueParser`] for parsing the value
///  - `options` is the name of the developer provided options in the action
///  - `value` is the name of the value in the action
///  - `action` is the action called to update the developer provided options
macro_rules! value_flag {
    ($short_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::None,
        )
    };

    (, $long_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal, $long_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::None,
        )
    };

    (, $long_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal, $long_name: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| Ok($action), $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal $description: literal ($parser: expr) => |$options: ident, $value: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ValueFlag::new($parser, |$options, $value| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };
}
