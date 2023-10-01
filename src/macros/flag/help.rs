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

#[test]
fn test() {
    let _: crate::FlagArgument<(), ()> = help_flag!("h", "help");
}
