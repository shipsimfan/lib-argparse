#[allow(unused_imports)]
use crate::FlagKind;

/// Creates a [`FlagKind::Help`]
///
/// Format:
///
/// `[short_name][, long_name] [no-exit|exit]`
///
/// `exit` makes this help flag exit after displaying.
/// `no-exit` makes this help flag return after displaying.
///
/// Arguments:
///  - `short_name` is the short name
///  - `long_name` is the long name
#[macro_export]
macro_rules! help_flag {
    ($short_name: literal) => {
        $crate::flag!($short_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    (, $long_name: literal) => {
        $crate::flag!(, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    ($short_name: literal, $long_name: literal) => {
        $crate::flag!($short_name, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    ($short_name: literal exit) => {
        $crate::flag!($short_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    (, $long_name: literal exit) => {
        $crate::flag!(, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    ($short_name: literal, $long_name: literal exit) => {
        $crate::flag!($short_name, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: true })
    };

    ($short_name: literal no-exit) => {
        $crate::flag!($short_name "Displays this help message" => $crate::FlagKind::Help { exit: false })
    };

    (, $long_name: literal no-exit) => {
        $crate::flag!(, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: false })
    };

    ($short_name: literal, $long_name: literal no-exit) => {
        $crate::flag!($short_name, $long_name "Displays this help message" => $crate::FlagKind::Help { exit: false })
    };
}