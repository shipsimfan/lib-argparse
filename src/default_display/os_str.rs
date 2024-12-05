use crate::DefaultDisplay;

impl DefaultDisplay for std::path::PathBuf {
    type Display<'a> = std::path::Display<'a>;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.display()
    }
}

impl DefaultDisplay for std::ffi::OsString {
    type Display<'a> = std::ffi::os_str::Display<'a>;

    fn as_display<'a>(&'a self) -> Self::Display<'a> {
        self.display()
    }
}
