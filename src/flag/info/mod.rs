use crate::Flag;

mod default;
mod display_help;
mod display_usage;
mod drop_default;

/// Information about a flag
///
/// Many types do not use some values in this struct
#[derive(Debug)]
pub struct FlagInfo<T: Flag> {
    /// The long name of the flag argument
    pub long_name: Option<&'static str>,

    /// The short name of the flag argument
    pub short_name: Option<&'static str>,

    /// The name of the value for this flag
    pub value: Option<&'static str>,

    /// The requested minimum length/quantity/value of arguments
    pub min: Option<f64>,

    /// The requested maximum length/quantity/value of arguments
    pub max: Option<f64>,

    /// The default value if none is provided by the user
    pub default: Option<fn() -> T>,

    /// A function which writes the description of this flag to stdout, taking a margin for printing new lines
    pub description: Option<fn(usize)>,
}
