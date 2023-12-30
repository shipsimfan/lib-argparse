mod parsing;
mod simple;
mod r#struct;
mod r#trait;

pub use parsing::ParsingPositionalArgument;
pub use r#struct::PositionalTerminalArgument;
pub use r#trait::PositionalArgument;
pub use simple::SimplePositionalArgument;
