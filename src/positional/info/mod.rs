use crate::Positional;

mod default;
mod display_help;
mod display_usage;
mod drop_default;

/// Information about a positional
///
/// Many types do not use some values in this struct
#[derive(Debug)]
pub struct PositionalInfo<T: Positional> {
    /// The name of the value for this positional
    pub value: &'static str,

    /// The requested minimum length/quantity/value of arguments
    pub min: Option<f64>,

    /// The requested maximum length/quantity/value of arguments
    pub max: Option<f64>,

    /// The default value if none is provided by the user
    pub default: Option<fn() -> T>,

    /// A function which writes the description of this positional to stdout, taking a margin for printing new lines
    pub description: Option<fn(usize)>,
}
