#[allow(unused_imports)]
use crate::FlagArgument;

#[macro_export]
#[doc(hidden)]
/// Creates a [`FlagArgument`]
///
/// Every selection can start with one of the following:
///  - `short_name `
///  - `, long name `
///  - `short_name, long_name `
/// Only the final form will be shown in the rest of this documentation
///
/// This is how the different forms translate to different action functions:
///  - `short_name, long_name description => |options| action` becomes `ActionFlag::simple()`
///  - `short_name, long_name description => |options|? action` becomes `ActionFlag::simple_res()`
///  - `short_name, long_name hint (missing) description => |options, &str| action` becomes `ActionFlag::str()`
///  - `short_name, long_name hint (missing) description => |options, &str|? action` becomes `ActionFlag::str_res()`
///  - `short_name, long_name hint (missing) description => |options, str| action` becomes `ActionFlag::string()`
///  - `short_name, long_name hint (missing) description => |options, str|? action` becomes `ActionFlag::string_res()`
///  - `short_name, long_name count*hint (missing) description => |options, str| action` becomes `ActionFlag::vec_string()`
///  - `short_name, long_name count*hint (missing) description => |options, str|? action` becomes `ActionFlag::vec_string_res()`
///
/// Arguments:
///  - `short_name` is the short name
///  - `long_name` is the long name
///  - `count` is the number of parameters to pull from the stream
///  - `hint` is the usage hint displayed in the help
///  - `missing` is the message displayed if the parameters are missing
///  - `description` is the description displayed in the help
///  - `options` is the name of the developer provided options variable in the action
///  - `str` is the name of the parameters variable in the action
macro_rules! action_flag {
    ($short_name: literal $description: literal => |$options: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple(|$options| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::None,
        )
    };

    (, $long_name: literal $description: literal => |$options: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple(|$options| $action, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal, $long_name: literal $description: literal => |$options: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple(|$options| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal $description: literal => |$options: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple_res(|$options| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::None,
        )
    };

    (, $long_name: literal $description: literal => |$options: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple_res(|$options| $action, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal, $long_name: literal $description: literal => |$options: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::simple_res(|$options| $action, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::None,
        )
    };

    ($short_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str(|$options, $str| $action, $missing, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, &$str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::str_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string(|$options, $str| $action, $missing, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::string_res(|$options, $str| $action, $missing, $description),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident| $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string_res(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::None,
            ::std::option::Option::Some($hint.into()),
        )
    };

    (, $long_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string_res(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::None,
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };

    ($short_name: literal, $long_name: literal $count: literal*$hint: literal ($missing: expr) $description: literal => |$options: ident, $str: ident|? $action: tt) => {
        $crate::macros::__name_flag(
            $crate::ActionFlag::vec_string_res(
                $count,
                |$options, $str| $action,
                $missing,
                $description,
            ),
            ::std::option::Option::Some($short_name.into()),
            ::std::option::Option::Some($long_name.into()),
            ::std::option::Option::Some($hint.into()),
        )
    };
}
