use r#struct::StructInput;

mod r#struct;

mod extract;
mod into_output;

/// The details extracted from the input item to the Command derive macro
pub enum Input {
    /// The input item is a struct
    Struct(StructInput),
}
