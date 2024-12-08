use crate::Positional;

mod default;
mod drop_default;
mod is_required;
mod multiple;

/// Information about a positional
///
/// Many types do not use some values in this struct
#[derive(Debug)]
pub struct PositionalInfo<T: Positional> {
    /// The name of the value for this positional
    pub value: &'static str,

    /// The requested minimum number of arguments
    pub min_count: usize,

    /// The requested maximum number of arguments (0 means infinite)
    pub max_count: usize,

    /// The default value if none is provided by the user
    pub default: Option<fn() -> T>,
}
