mod default;

/// Information about a flag
///
/// Many types do not use some values in this struct
#[derive(Debug)]
pub struct FlagInfo<T> {
    /// The long name of the flag argument
    pub long_name: Option<&'static str>,

    /// The short name of the flag argument
    pub short_name: Option<&'static str>,

    /// The name of the value for this flag
    pub value: Option<&'static str>,

    /// The requested minimum number of arguments
    pub min_count: usize,

    /// The requested maximum number of arguments (0 means infinite)
    pub max_count: usize,

    /// The default value if none is provided by the user
    pub default: Option<fn() -> T>,
}
