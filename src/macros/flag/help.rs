#[allow(unused_imports)]
use crate::FlagKind;

/// Creates a [`FlagKind::Help`]
///
/// Format:
///
/// `[short_name][, long_name]`
///
/// Arguments:
///  - `short_name` is the short name
///  - `long_name` is the long name
#[macro_export]
macro_rules! help_flag {
    ($short_name: literal) => {
        $crate::flag!($short_name "Displays this help message" => $crate::FlagKind::Help)
    };

    (, $long_name: literal) => {
        $crate::flag!(, $long_name "Displays this help message" => $crate::FlagKind::Help)
    };

    ($short_name: literal, $long_name: literal) => {
        $crate::flag!($short_name, $long_name "Displays this help message" => $crate::FlagKind::Help)
    };
}
